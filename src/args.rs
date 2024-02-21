use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub(super) struct Args {
  #[clap(
    long,
    short,
    env = "SFLOW_EXPORTER_META",
    default_value = "meta.yaml"
  )]
  pub(super) meta: PathBuf,
  #[clap(
    long,
    short,
    env = "SFLOW_EXPORTER_SFlOW_LISTEN_ADDR",
    default_value = "[::]:6343"
  )]
  pub(super) sflow_listen_addr: SocketAddr,
  #[clap(
    long,
    env = "SFLOW_EXPORTER_METRICS_LISTEN_ADDR",
    default_value = "[::]:9100"
  )]
  pub(super) metrics_listen_addr: SocketAddr,
}
