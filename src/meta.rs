use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use crate::sflow::IpAddr;

pub(super) const DEFAULT_ETHER_TYPE: &str = "other";

pub(super) struct Meta {
  routers: HashMap<[u8; 6], Router>,
  agents: HashMap<IpAddr, Agent>,
  ether_types: HashMap<u16, String>,
}

pub(super) struct Router {
  pub(super) label: String,
}

pub(super) struct Agent {
  pub(super) label: String,
}

#[derive(Deserialize)]
struct MetaStorage {
  routers: Vec<RouterStorage>,
  agents: Vec<AgentStorage>,
  ether_types: HashMap<u16, EtherTypeStorage>,
}

#[derive(Deserialize)]
struct RouterStorage {
  mac: String,
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
    let meta = serde_yaml_ng::from_str::<MetaStorage>(&raw_meta)?;

    let routers = meta
      .routers
      .into_iter()
      .map(|customer| {
        (
          convert_mac(&customer.mac),
          Router {
            label: customer.label,
          },
        )
      })
      .collect();

    let agents = meta
      .agents
      .into_iter()
      .map(|agent| (agent.source, Agent { label: agent.label }))
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

  pub(super) fn router_count(&self) -> usize {
    self.routers.len()
  }

  pub(super) fn agent_count(&self) -> usize {
    self.agents.len()
  }

  pub(super) fn important_ether_type_count(&self) -> usize {
    self.ether_types.len()
  }

  pub(super) fn get_agents(&self) -> Values<'_, IpAddr, Agent> {
    self.agents.values()
  }

  pub(super) fn get_routers(&self) -> Values<'_, [u8; 6], Router> {
    self.routers.values()
  }

  pub(super) fn get_ether_types(&self) -> Values<'_, u16, String> {
    self.ether_types.values()
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
