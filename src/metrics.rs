use prometheus::{IntCounterVec, Opts, Registry};

pub(super) struct Metrics {
  router_bytes: IntCounterVec,
  agent_bytes: IntCounterVec,
  agent_drops: IntCounterVec,
}

impl Metrics {
  pub(super) fn new() -> (Self, Registry) {
    let router_bytes = IntCounterVec::new(
      Opts::new("sflow_router_bytes", "bytes"),
      &["agent", "in", "out", "ether_type"],
    )
    .unwrap();
    let agent_bytes = IntCounterVec::new(
      Opts::new("sflow_agent_bytes", "bytes"),
      &["agent", "ether_type"],
    )
    .unwrap();
    let agent_drops =
      IntCounterVec::new(Opts::new("sflow_agent_drops", "drops"), &["agent"]).unwrap();

    let registry = Registry::new();
    registry.register(Box::new(router_bytes.clone())).unwrap();
    registry.register(Box::new(agent_bytes.clone())).unwrap();
    registry.register(Box::new(agent_drops.clone())).unwrap();

    (
      Self {
        router_bytes,
        agent_bytes,
        agent_drops,
      },
      registry,
    )
  }

  pub(super) fn capture_router_bytes(
    &self,
    agent: &str,
    r#in: &str,
    r#out: &str,
    ether_type: &str,
    bytes: u64,
  ) {
    self
      .router_bytes
      .with_label_values(&[agent, r#in, r#out, ether_type])
      .inc_by(bytes);
  }

  pub(super) fn capture_agent_bytes(&self, agent: &str, ether_type: &str, bytes: u64) {
    self
      .agent_bytes
      .with_label_values(&[agent, ether_type])
      .inc_by(bytes);
  }

  pub(super) fn capture_pagent_drops(&self, agent: &str, drops: u32) {
    self
      .agent_drops
      .with_label_values(&[agent])
      .inc_by(drops as u64);
  }
}
