use std::fmt::{Display, Formatter};
use std::io::{Read, Seek};
use std::net::{Ipv4Addr, Ipv6Addr};

use binrw::{BinRead, BinResult, Endian};
use serde::Deserialize;

use crate::sflow::sample::Sample;
use crate::sflow::IpAddr::{IPv4, IPv6};

pub(crate) mod record;
pub(crate) mod sample;

#[derive(BinRead)]
pub(crate) struct SflowDatagram {
  #[brw(assert(version == 5))]
  pub(crate) version: u32,
  pub(crate) agent_addr: IpAddr,
  pub(crate) sub_agent_id: u32,
  pub(crate) seq_num: u32,
  pub(crate) uptime: u32,
  sample_count: u32,
  #[br(count = sample_count)]
  pub(crate) samples: Vec<Sample>,
}
#[derive(Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub(crate) enum IpAddr {
  IPv4(Ipv4Addr),
  IPv6(Ipv6Addr),
}

impl Display for IpAddr {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      IPv4(v4) => Display::fmt(v4, f),
      IPv6(v6) => Display::fmt(v6, f),
    }
  }
}

impl BinRead for IpAddr {
  type Args<'a> = ();

  fn read_options<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    args: Self::Args<'_>,
  ) -> BinResult<Self> {
    match u32::read_options(reader, endian, args)? {
      1 => Ok(IPv4(Ipv4Addr::from(u32::read_options(
        reader, endian, args,
      )?))),
      2 => Ok(IPv6(Ipv6Addr::from(u128::read_options(
        reader, endian, args,
      )?))),
      magic => Err(binrw::Error::BadMagic {
        pos: reader.stream_position()?,
        found: Box::new(magic),
      }),
    }
  }
}
