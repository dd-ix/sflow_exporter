use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about)]
pub(super) struct Args {
  #[clap(long, short, env = "SFLOW_EXPORTER_META", default_value = "meta.yaml")]
  pub(super) meta: PathBuf,
  #[clap(subcommand)]
  pub(super) action: Action,
}

#[derive(Subcommand)]
pub(super) enum Action {
  Check,
  Listen {
    #[clap(
      long,
      short,
      env = "SFLOW_EXPORTER_SFlOW_LISTEN_ADDR",
      default_value = "[::]:6343"
    )]
    sflow_addr: SocketAddr,
    #[clap(
      long,
      env = "SFLOW_EXPORTER_METRICS_LISTEN_ADDR",
      default_value = "[::]:9144"
    )]
    metrics_addr: SocketAddr,
  },
}
