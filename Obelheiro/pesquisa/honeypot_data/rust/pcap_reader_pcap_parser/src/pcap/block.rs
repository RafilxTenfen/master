use chrono::NaiveDateTime;
use etherparse::SlicedPacket;
use pcap_parser::PcapBlockOwned::{Legacy, LegacyHeader, NG};
use pcap_parser::{LegacyPcapBlock, PcapBlockOwned};

mod dns;
mod ip;
mod ldap;
mod ntp;
mod udp;

pub struct PcapPacket {
  pub id: i32,
  pub timestamp: NaiveDateTime,
  pub ip: ip::PcapIP,
  pub udp: udp::PcapUDP,
  pub attack_type: PcapAttackType,
  pub dns: Option<dns::PcapDNS>,
  pub ldap: Option<ldap::PcapLDAP>,
  pub ntp: Option<ntp::PcapNTP>,
}

pub enum PcapAttackType {
  // None,
  // CHARGEN,
  // SSDP,
  DNS,
  NTP,
  LDAP,
}

impl PcapAttackType {
  pub fn to_string(&self) -> String {
    match self {
      // PcapAttackType::None => String::from("None"),
      // PcapAttackType::CHARGEN => String::from("CHARGEN"),
      // PcapAttackType::SSDP => String::from("SSDP"),
      PcapAttackType::DNS => String::from("DNS"),
      PcapAttackType::LDAP => String::from("LDAP"),
      PcapAttackType::NTP => String::from("NTP"),
    }
  }
}

pub fn process_block(block: &PcapBlockOwned) -> Option<PcapPacket> {
  match block {
    LegacyHeader(header) => {
      println!("PCAP Header size {}", header.size());
      return None;
    }
    Legacy(block) => {
      return process_legacy_block(block);
    }
    NG(new_block) => {
      println!("NG new block in pcap {}", new_block.magic());
      return None;
    }
  }
}

fn process_legacy_block(b: &LegacyPcapBlock) -> Option<PcapPacket> {
  let naive_date_time = NaiveDateTime::from_timestamp(i64::from(b.ts_sec), b.ts_usec);

  match SlicedPacket::from_ethernet(b.data) {
    Ok(sliced_packet) => {
      return process_sliced_packet(sliced_packet, naive_date_time);
    }
    Err(err) => {
      println!("Err SlicedPacket::from_ethernet {}", err);
      return None;
    }
  }
}

fn process_sliced_packet(
  sliced_packet: SlicedPacket,
  timestamp: NaiveDateTime,
) -> Option<PcapPacket> {
  let ip = match sliced_packet.ip {
    Some(ip) => match ip {
      etherparse::InternetSlice::Ipv4(ipv4_header, _) => {
        ip::process_ip(ipv4_header, 0)
      }
      etherparse::InternetSlice::Ipv6(_, _) => {
        println!("Parsed etherparse::InternetSlice::Ipv6");
        return None;
      }
    },
    None => {
      println!("Err parse IP");
      return None;
    }
  };

  let udp = match sliced_packet.transport {
    Some(transport_slice) => match transport_slice {
      etherparse::TransportSlice::Udp(ref udp_slice) => {
        udp::process_udp(udp_slice, 0)
      }
      etherparse::TransportSlice::Icmpv4(_) => {return None}
      etherparse::TransportSlice::Icmpv6(_) => {return None}
      etherparse::TransportSlice::Tcp(_) => {return None}
      etherparse::TransportSlice::Unknown(_) => {return None}
    },
    None => {return None;}
  };

  let mut packet = PcapPacket{
    id: 0,
    ip,
    udp,
    timestamp,
    attack_type: PcapAttackType::NTP,
    ntp: None,
    dns: None,
    ldap: None,
  };

  match ntp_parser::parse_ntp(sliced_packet.payload) {
    Ok((_, ref ntp_packet)) => {
      packet.ntp = Some(ntp::process_ntp(ntp_packet, 0));
      return Some(packet);
    }
    Err(_) => {}
  }

  match ldap_parser::parse_ldap_messages(sliced_packet.payload) {
    Ok((_, ref ldap)) => {
      for msg in ldap {
        packet.ldap = Some(ldap::process_ldap(msg, 0));
        packet.attack_type = PcapAttackType::LDAP;
        // we only care for the first msg
        return Some(packet);
      }
    }
    Err(_) => {}
  }

  match dns_parser::Packet::parse(sliced_packet.payload) {
    Ok(ref dns_packet) => {
      packet.dns = Some(dns::process_dns(dns_packet, 0));
      packet.attack_type = PcapAttackType::DNS;
      return Some(packet);
    }
    Err(_) => {}
  }

  return None;
}
