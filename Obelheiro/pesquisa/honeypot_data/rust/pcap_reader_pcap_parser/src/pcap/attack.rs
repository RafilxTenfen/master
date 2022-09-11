use chrono::NaiveDateTime;
use cidr_utils::cidr::Ipv4Cidr;
// use etherparse::SlicedPacket;
// use pcap_parser::PcapBlockOwned::{Legacy, LegacyHeader, NG};
// use pcap_parser::{LegacyPcapBlock, PcapBlockOwned};

use super::block::PcapPacket;

pub struct PcapAttack {
  id: i32,
  ip_vitima_cidr: Ipv4Cidr,
  packets: Vec<PcapPacket>,
  timestamp_inicio: NaiveDateTime,
  timestamp_fim: NaiveDateTime,
}

pub fn process_packet(new_packet: &PcapPacket) {

  // new_packet.
}