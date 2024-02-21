use std::io::{Read, Seek};

use binrw::{BinRead, BinResult, Endian};

use crate::sflow::record::FlowRecord;

#[derive(BinRead)]
pub(crate) enum Sample {
  #[brw(magic = 1u32)]
  Flow(FlowData),
  // #[brw(magic = 2u32)]
  // Counter(CounterData),
  // #[brw(magic = 3u32)]
  // FlowExpanded(FlowExtendedData),
  // #[brw(magic = 4u32)]
  // CounterExpanded(CounterExtendedData),
  Unknown(UnknownData),
}

pub(crate) struct FlowData {
  pub(crate) seq_num: u32,
  pub(crate) source_id_idx: u32,
  pub(crate) source_id_type: u32,
  pub(crate) sample_rate: u32,
  pub(crate) sample_pool: u32,
  pub(crate) drops: u32,
  pub(crate) input_if_idx: u32,
  pub(crate) input_if_format: u32,
  pub(crate) output_if_idx: u32,
  pub(crate) output_if_format: u32,
  pub(crate) direction: Direction,
  pub(crate) records: Vec<FlowRecord>,
}

#[derive(BinRead)]
struct FlowDataRaw {
  data_len: u32,
  seq_num: u32,
  source_id: u32,
  sample_rate: u32,
  sample_pool: u32,
  drops: u32,
  input_if_idx: u32,
  output_if_idx: u32,
  record_count: u32,
  #[br(count = record_count)]
  records: Vec<FlowRecord>,
}

// #[derive(BinRead, Debug)]
// pub(crate) struct CounterData {
//     data_len: u32,
//     #[br(count = data_len)]
//     data: Vec<u8>,
//     // seq_num: u32,
//     // source_id: u32,
//     // counter_count: u32,
//     // #[br(count = counter_count)]
//     // records: Vec<CounterRecord>,
// }
//
// #[derive(BinRead, Debug)]
// pub(crate) struct FlowExtendedData {
//     data_len: u32,
//     #[br(count = data_len)]
//     data: Vec<u8>,
// }
//
// #[derive(BinRead, Debug)]
// pub(crate) struct CounterExtendedData {
//     data_len: u32,
//     #[br(count = data_len)]
//     data: Vec<u8>,
// }

#[derive(BinRead)]
pub(crate) struct UnknownData {
  pub(crate) magic: u32,
  data_len: u32,
  #[br(count = data_len)]
  data: Vec<u8>,
}

pub(crate) enum Direction {
  Ingress,
  Egress,
  Unknown,
}

impl BinRead for FlowData {
  type Args<'a> = ();

  fn read_options<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    _args: Self::Args<'_>,
  ) -> BinResult<Self> {
    let raw: FlowDataRaw = FlowDataRaw::read_options(reader, endian, ())?;

    let source_id_idx = raw.source_id & 0x00ffffff;

    Ok(Self {
      seq_num: raw.seq_num,
      source_id_idx,
      source_id_type: raw.source_id >> 24,
      sample_rate: raw.sample_rate,
      sample_pool: raw.sample_pool,
      drops: raw.drops,
      input_if_idx: raw.input_if_idx & 0x3FFFFFFF,
      input_if_format: raw.input_if_idx >> 30,
      output_if_idx: raw.output_if_idx & 0x3FFFFFFF,
      output_if_format: raw.output_if_idx >> 30,
      direction: match source_id_idx {
        idx if idx == raw.output_if_idx => Direction::Egress,
        idx if idx == raw.input_if_idx => Direction::Ingress,
        _ => Direction::Unknown,
      },
      records: raw.records,
    })
  }
}
