use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use binrw::{BinRead, Endian};
use clap::Parser;
use futures_util::stream::StreamExt;
use inotify::{Inotify, WatchMask};
use prometheus::{Registry, TextEncoder};
use std::future::IntoFuture;
use std::io::Cursor;
use std::path::PathBuf;
use tokio::net::{TcpListener, UdpSocket};
use tokio::select;
use tokio::sync::mpsc;
use tracing::error;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::args::Args;
use crate::meta::Meta;
use crate::metrics::Metrics;
use crate::sflow::record::{EthernetHeader, FlowRecord, HeaderProtocol};
use crate::sflow::sample::Sample;
use crate::sflow::SflowDatagram;
use crate::utils::datagram_buffer;
use crate::utils::shutdown_signal;

mod args;

mod meta;
mod metrics;
mod sflow;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .compact()
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  info!(concat!(
    "Booting ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "..."
  ));

  let (meta_update_tx, meta_update_rx) = mpsc::channel(10);

  let (metrics, registry) = Metrics::new();

  let socket = UdpSocket::bind(args.sflow_listen_addr).await?;
  info!("sflow listening at {}/udp...", args.sflow_listen_addr);

  let listener = TcpListener::bind(args.metrics_listen_addr).await?;
  info!(
    "metrics listening at http://{}/metrics...",
    args.metrics_listen_addr
  );

  let router = axum::Router::new()
    .route("/metrics", get(metrics_endpoint))
    .with_state(registry)
    .into_make_service();

  let inotify = {
    let meta = args.meta.clone();
    tokio::spawn(async move {
      let inotify = Inotify::init()?;
      inotify.watches().add(meta, WatchMask::MODIFY)?;
      let mut buf = [0; 1024];
      let mut stream = inotify.into_event_stream(&mut buf)?;

      while let Some(_) = stream.next().await.transpose()? {
        meta_update_tx.send(()).await.unwrap()
      }

      Ok::<(), anyhow::Error>(())
    })
  };

  let handle = tokio::spawn(process_sflow(socket, meta_update_rx, args.meta, metrics));
  let axum = axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .into_future();

  select! {
    result = axum => { result? }
    result = handle => { result?? }
    result = inotify => { result?? }
  }

  Ok(())
}

async fn process_sflow(
  socket: UdpSocket,
  mut meta_update_rx: mpsc::Receiver<()>,
  meta_path: PathBuf,
  metrics: Metrics,
) -> anyhow::Result<()> {
  let mut buf = datagram_buffer();
  let mut meta = Meta::load(&meta_path).await?;
  info!(
    "Loaded {} customers, {} agents and {} important ether types",
    meta.customer_count(),
    meta.agent_count(),
    meta.important_ether_type_count()
  );

  loop {
    let read = select! {
      _ = meta_update_rx.recv() => {
        match Meta::load(&meta_path).await {
          Ok(new_meta) => {
            meta = new_meta;
            info!(
              "Loaded {} customers, {} agents and {} important ether types",
              meta.customer_count(),
              meta.agent_count(),
              meta.important_ether_type_count()
            );
          }
          Err(err) => {
            error!("Unable to load meta configuration, continuing with running configuration: {:?}", err);
          }
        };
        continue;
      }
      result = socket.recv(buf.as_mut_slice()) => { result? }
    };

    let mut cursor = Cursor::new(&buf[..read]);

    let datagram = SflowDatagram::read_options(&mut cursor, Endian::Big, ())?;

    let agent = match meta.lookup_agent(&datagram.agent_addr) {
      Some(agent) => agent,
      None => continue,
    };

    for sample in datagram.samples {
      let flow = match sample {
        Sample::Flow(flow) => flow,
        _ => continue,
      };

      metrics.capture_pagent_drops(&agent.label, flow.drops);

      for record in flow.records {
        let packet_header = match record {
          FlowRecord::RawPacketHeader(header) => header,
          _ => continue,
        };

        let ethernet_header = match packet_header.protocol_header {
          HeaderProtocol::Ethernet(EthernetHeader(header)) => header,
          _ => continue,
        };

        // first cast, then multiply to prevent overflow (panic!)
        let bytes = packet_header.frame_length as u64 * flow.sample_rate as u64;
        let ether_type = meta.fmt_ether_type(ethernet_header.ether_type.0);

        metrics.capture_agent_bytes(&agent.label, ether_type, bytes);

        let src = meta.lookup_router(&ethernet_header.source);
        let dst = meta.lookup_router(&ethernet_header.destination);

        if let (Some(src), Some(dst)) = (src, dst) {
          if src.agent != agent.id || src.interface != flow.input_if_idx {
            continue;
          }

          metrics.capture_router_bytes(&agent.label, &src.label, &dst.label, ether_type, bytes);
        }
      }
    }
  }
}

async fn metrics_endpoint(State(registry): State<Registry>) -> Result<String, StatusCode> {
  let encoder = TextEncoder::new();
  match encoder.encode_to_string(&registry.gather()) {
    Ok(metrics) => Ok(metrics),
    Err(err) => {
      error!("Error encoding metrics: {:?}", err);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
