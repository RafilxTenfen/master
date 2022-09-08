use chrono::{DateTime, FixedOffset, NaiveDateTime};
use etherparse::SlicedPacket;
use libpcap_tools::PcapBlockOwned;
use libpcap_tools::PcapBlockOwned::{Legacy, LegacyHeader, NG};
use pcap_parser::{parse_pcap_frame, LegacyPcapBlock};
// use pktparse::ethernet;
mod dns;
mod ip;
mod udp;
mod ntp;

pub fn process_block(block: &PcapBlockOwned) {
  match block {
    LegacyHeader(header) => {
      println!("PCAP Header size {}", header.size());
    }
    Legacy(block) => {
      process_legacy_block(block);
    }
    NG(new_block) => {
      println!("NG new block in pcap {}", new_block.magic());
    }
  }
}

fn process_legacy_block(b: &LegacyPcapBlock) {
  // println!("process_legacy_block: {}, {}", b.ts_sec, b.ts_usec);

  let naive_date_time = NaiveDateTime::from_timestamp(i64::from(b.ts_sec), b.ts_usec);

  // println!("process_legacy_block: {}, {} - {}", b.ts_sec, b.ts_usec, naive_date_time.to_string());

  // DateTime::naive_local(&self)
  // match ethernet::parse_ethernet_frame(b.data) {
  //   Ok((_, ethernet_frame)) => {
  //     // println!("ETHERNET parsed",);

  //   }
  //   Err(err) => {
  //     println!("Err parse_ethernet_frame {}", err)
  //   }
  // }

  match SlicedPacket::from_ethernet(b.data) {
    Ok(sliced_packet) => {
      process_sliced_packet(sliced_packet);
    }
    Err(err) => {
      println!("Err SlicedPacket::from_ethernet {}", err)
    }
  }
}

fn process_sliced_packet(sliced_packet: SlicedPacket) {
  match sliced_packet.ip {
    Some(ip) => match ip {
      etherparse::InternetSlice::Ipv4(ipv4_header, _) => {
        ip::process_ip(ipv4_header, 0);
      }
      etherparse::InternetSlice::Ipv6(_, _) => {}
    },
    None => {}
  }

  match sliced_packet.transport {
    Some(transport_slice) => match transport_slice {
      etherparse::TransportSlice::Udp(ref udp_slice) => {
        udp::process_udp(udp_slice, 0);

        match ntp_parser::parse_ntp(sliced_packet.payload) {
          Ok((_, ref ntp_packet)) => {
            ntp::process_ntp(ntp_packet, 0);
          }
          Err(err) => {
            // println!("failed ntp_parser::parse_ntp {}", err)
          }
        }

        match dns_parser::Packet::parse(sliced_packet.payload) {
          Ok(ref dns_packet) => {
            dns::process_dns(dns_packet, 0);
          }
          Err(_err) => {
            // println!("Err dns_parser::Packet::parse {}", err)
          }
        }
      }
      etherparse::TransportSlice::Icmpv4(_icmpv4_slice) => {}
      etherparse::TransportSlice::Icmpv6(_icmpv6_slice) => {}
      etherparse::TransportSlice::Tcp(ref _tcp_slice) => {}
      etherparse::TransportSlice::Unknown(_unknown) => {}
    },
    None => {}
  }
}
