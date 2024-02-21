use std::io::{Read, Seek, SeekFrom};

use binrw::{BinRead, BinResult, Endian};
use etherparse::Ethernet2Header;

#[derive(BinRead, Debug)]
pub(crate) enum FlowRecord {
  #[brw(magic = 1u32)]
  RawPacketHeader(RawPacketHeaderData),
  // #[brw(magic = 2u32)]
  // EthernetFrame {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 3u32)]
  // Ipv4 {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 4u32)]
  // Ipv6 {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1001u32)]
  // ExtendedSwitch {
  //     data_len: u32,
  //     src_vlan: u32,
  //     src_priority: u32,
  //     dst_vlan: u32,
  //     dst_priority: u32,
  // },
  // #[brw(magic = 1002u32)]
  // ExtendedRouter {
  //     data_len: u32,
  //     next_hop: IpAddr,
  //     src_mask_len: u32,
  //     dst_mask_len: u32,
  // },
  // #[brw(magic = 1003u32)]
  // ExtendedGateway {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1004u32)]
  // ExtendedUser {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1005u32)]
  // ExtendedUrl {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1006u32)]
  // ExtendedMpls {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1006u32)]
  // ExtendedNat {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1008u32)]
  // ExtendedMplsTunnel {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1009u32)]
  // ExtendedMplsVc {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1010u32)]
  // ExtendedMplsFec {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1011u32)]
  // ExtendedMplsLvpFec {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  // #[brw(magic = 1012u32)]
  // ExtendedVlanTunnel {
  //     data_len: u32,
  //     #[br(count = data_len)]
  //     data: Vec<u8>,
  // },
  Unknown {
    magic: u32,
    data_len: u32,
    #[br(count = data_len)]
    data: Vec<u8>,
  },
}

// #[derive(BinRead, Debug)]
// pub(crate) enum CounterRecord {
//     #[brw(magic = 1u32)]
//     GenericInterface {
//         data_len: u32,
//         #[br(count = data_len)]
//         data: Vec<u8>,
//     },
//     #[brw(magic = 2u32)]
//     EthernetInterface {
//         data_len: u32,
//         #[br(count = data_len)]
//         data: Vec<u8>,
//     },
//     #[brw(magic = 3u32)]
//     TokenRing {
//         data_len: u32,
//         #[br(count = data_len)]
//         data: Vec<u8>,
//     },
//     #[brw(magic = 4u32)]
//     _100BaseVGInterface {
//         data_len: u32,
//         #[br(count = data_len)]
//         data: Vec<u8>,
//     },
//     #[brw(magic = 5u32)]
//     Vlan {
//         data_len: u32,
//         #[br(count = data_len)]
//         data: Vec<u8>,
//     },
//     #[brw(magic = 1001u32)]
//     Processor {
//         data_len: u32,
//         #[br(count = data_len)]
//         data: Vec<u8>,
//     },
//     Unknown {
//         magic: u32,
//         data_len: u32,
//         #[br(count = data_len)]
//         data: Vec<u8>,
//     },
// }

#[derive(BinRead)]
enum HeaderProtocolRaw {
  #[brw(magic = 1u32)]
  EthernetISO88023,
  // #[brw(magic = 2u32)]
  // ISO88024TokenBus,
  // #[brw(magic = 3u32)]
  // ISO88025TokenRing,
  // #[brw(magic = 4u32)]
  // FDDI,
  // #[brw(magic = 5u32)]
  // FrameRelay,
  // #[brw(magic = 6u32)]
  // X25,
  // #[brw(magic = 7u32)]
  // PPP,
  // #[brw(magic = 8u32)]
  // SMDS,
  // #[brw(magic = 9u32)]
  // AAL5,
  // /* e.g. Cisco AAL5 mux */
  // #[brw(magic = 10u32)]
  // AAL5IP,
  // #[brw(magic = 11u32)]
  // IPv4,
  // #[brw(magic = 12u32)]
  // IPv6,
  // #[brw(magic = 13u32)]
  // MPLS,
  // /* RFC 1662, 2615 */
  // #[brw(magic = 14u32)]
  // POS,
  // Unknown {
  //     magic: u32,
  // },
}

#[derive(Debug)]
pub(crate) enum HeaderProtocol {
  Ethernet(EthernetHeader),
  // ISO88024TokenBus,
  // ISO88025TokenRing,
  // FDDI,
  // FrameRelay,
  // X25,
  // PPP,
  // SMDS,
  // AAL5,
  // /* e.g. Cisco AAL5 mux */
  // AAL5IP,
  // IPv4,
  // IPv6,
  // MPLS,
  // /* RFC 1662, 2615 */
  // POS,
  Unknown { magic: u32 },
}
//
// #[derive(Debug)]
// pub(crate) enum EthernetPayload {
//     Ipv4(Ipv4Header),
//     Ipv6(Ipv6Header),
//     Unknown,
// }

//
// impl BinRead for EthernetHeader {
//     type Args<'a> = (u32,);
//
//     fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
//         let mut buf = vec![0u8; args.0 as usize];
//         reader.read_exact(&mut buf[..args.0 as usize])?;
//
//         let mut cursor = Cursor::new(&buf);
//
//         let ethernet = Ethernet2Header::read(&mut cursor)?;
//
//         Ok(EthernetHeader {
//             ethernet,
//         })
//     }
// }
#[derive(Debug)]
pub(crate) struct RawPacketHeaderData {
  pub(crate) frame_length: u32,
  pub(crate) stripped_octets: u32,
  pub(crate) protocol_header: HeaderProtocol,
}

#[derive(BinRead)]
struct RawPacketHeaderDataRaw {
  data_len: u32,
  protocol: HeaderProtocolRaw,
  frame_length: u32,
  stripped_octets: u32,
  header_length: u32,
}

#[derive(Debug)]
pub(crate) struct EthernetHeader(
  pub(crate) Ethernet2Header, // payload: EthernetPayload,
);

impl BinRead for RawPacketHeaderData {
  type Args<'a> = ();

  fn read_options<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    _args: Self::Args<'_>,
  ) -> BinResult<Self> {
    let raw: RawPacketHeaderDataRaw = RawPacketHeaderDataRaw::read_options(reader, endian, ())?;

    let header = raw.data_len - 4 * 4;

    let protocol = match raw.protocol {
      HeaderProtocolRaw::EthernetISO88023 => {
        HeaderProtocol::Ethernet(EthernetHeader::read_options(reader, endian, header)?)
      } // HeaderProtocolRaw::ISO88024TokenBus => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::ISO88024TokenBus
        // }
        // HeaderProtocolRaw::ISO88025TokenRing => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::ISO88025TokenRing
        // }
        // HeaderProtocolRaw::FDDI => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::FDDI
        // }
        // HeaderProtocolRaw::FrameRelay => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::FrameRelay
        // }
        // HeaderProtocolRaw::X25 => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::X25
        // }
        // HeaderProtocolRaw::PPP => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::PPP
        // }
        // HeaderProtocolRaw::SMDS => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::SMDS
        // }
        // HeaderProtocolRaw::AAL5 => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::AAL5
        // }
        // HeaderProtocolRaw::AAL5IP => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::AAL5IP
        // }
        // HeaderProtocolRaw::IPv4 => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::IPv4
        // }
        // HeaderProtocolRaw::IPv6 => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::IPv6
        // }
        // HeaderProtocolRaw::MPLS => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::MPLS
        // }
        // HeaderProtocolRaw::POS => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::POS
        // }
        // HeaderProtocolRaw::Unknown { magic } => {
        //     reader.seek(SeekFrom::Current(header as i64))?;
        //     HeaderProtocol::Unknown { magic }
        // }
    };

    Ok(RawPacketHeaderData {
      frame_length: raw.frame_length,
      stripped_octets: raw.stripped_octets,
      protocol_header: protocol,
    })
  }
}

impl BinRead for EthernetHeader {
  type Args<'a> = u32;

  fn read_options<R: Read + Seek>(
    reader: &mut R,
    _endian: Endian,
    args: Self::Args<'_>,
  ) -> BinResult<Self> {
    let pos = reader.stream_position()?;
    let header = Ethernet2Header::read(reader)?;
    reader.seek(SeekFrom::Start(pos + args as u64))?;

    // let ether_type = header.ether_type.clone();
    Ok(EthernetHeader(
      header,
      // payload: {
      //     let payload = match ether_type {
      //         EtherType::IPV4 => EthernetPayload::Ipv4(Ipv4Header::read(reader).unwrap()),
      //         EtherType::IPV6 => EthernetPayload::Ipv6(Ipv6Header::read(reader).unwrap()),
      //         _ => EthernetPayload::Unknown,
      //     };
      //     reader.seek(SeekFrom::Start(pos + args as u64))?;
      //     payload
      // },
    ))
  }
}
