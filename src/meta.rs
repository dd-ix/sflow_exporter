use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use crate::sflow::IpAddr;

const DEFAULT_ETHER_TYPE: &str = "other";

pub(super) struct Meta {
  routers: HashMap<[u8; 6], Router>,
  agents: HashMap<IpAddr, Agent>,
  ether_types: HashMap<u16, String>,
}

pub(super) struct Router {
  pub(super) agent: String,
  pub(super) interface: u32,
  pub(super) label: String,
}

pub(super) struct Agent {
  pub(super) id: String,
  pub(super) label: String,
}

#[derive(Deserialize)]
struct MetaStorage {
  routers: Vec<CustomerStorage>,
  agents: HashMap<String, AgentStorage>,
  ether_types: HashMap<u16, EtherTypeStorage>,
}

#[derive(Deserialize)]
struct CustomerStorage {
  mac: String,
  agent: String,
  interface: u32,
  label: String,
}

#[derive(Deserialize)]
struct AgentStorage {
  label: String,
  source: IpAddr,
}

#[derive(Deserialize)]
struct EtherTypeStorage {
  label: String,
}

impl Meta {
  pub(super) async fn load(path: &Path) -> anyhow::Result<Self> {
    let raw_meta = tokio::fs::read_to_string(path).await?;
    let meta = serde_yaml::from_str::<MetaStorage>(&raw_meta)?;

    let routers = meta
      .routers
      .into_iter()
      .map(|customer| {
        (
          convert_mac(&customer.mac),
          Router {
            label: customer.label,
            agent: customer.agent,
            interface: customer.interface,
          },
        )
      })
      .collect();

    let agents = meta
      .agents
      .into_iter()
      .map(|(id, agent)| {
        (
          agent.source,
          Agent {
            id,
            label: agent.label,
          },
        )
      })
      .collect();

    let ether_types = meta
      .ether_types
      .into_iter()
      .map(|(id, ether_type)| (id, ether_type.label))
      .collect();

    Ok(Self {
      routers,
      agents,
      ether_types,
    })
  }

  pub(super) fn customer_count(&self) -> usize {
    self.routers.len()
  }

  pub(super) fn agent_count(&self) -> usize {
    self.agents.len()
  }

  pub(super) fn important_ether_type_count(&self) -> usize {
    self.ether_types.len()
  }

  pub(super) fn lookup_router(&self, mac: &[u8; 6]) -> Option<&Router> {
    self.routers.get(mac)
  }

  pub(super) fn lookup_agent(&self, addr: &IpAddr) -> Option<&Agent> {
    self.agents.get(addr)
  }

  pub(super) fn fmt_ether_type(&self, ether_type: u16) -> &str {
    self
      .ether_types
      .get(&ether_type)
      .map(|value| value.as_str())
      .unwrap_or(DEFAULT_ETHER_TYPE)
  }
}

fn convert_mac(input: &str) -> [u8; 6] {
  let mac = input
    .split(':')
    .map(|byte| u8::from_str_radix(byte, 16).unwrap())
    .collect::<Vec<_>>();

  if mac.len() != 6 {
    panic!("Invalid mac addr: {}", input);
  }

  [mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]]
}
