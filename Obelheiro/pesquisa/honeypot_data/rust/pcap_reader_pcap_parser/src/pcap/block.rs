use chrono::NaiveDateTime;
use cidr_utils::cidr::Ipv4Cidr;
use etherparse::SlicedPacket;
use pcap_parser::PcapBlockOwned::{Legacy, LegacyHeader, NG};
use pcap_parser::{LegacyPcapBlock, PcapBlockOwned};
use rusqlite::{params, Connection};
// use tokio_postgres::Client;
// use rusqlite::{params, Connection};
use std::collections::HashMap;

mod dns;
mod ip;
mod ldap;
mod ntp;
mod udp;

pub struct PcapPacket {
  pub id: u32,
  pub timestamp: NaiveDateTime,
  pub ip: ip::PcapIP,
  pub udp: udp::PcapUDP,
  pub attack_type: PcapAttackType,
  pub dns: Option<dns::PcapDNS>,
  pub ldap: Option<ldap::PcapLDAP>,
  pub ntp: Option<ntp::PcapNTP>,
}

pub enum PcapAttackType {
  STEAM,
  SSDP,
  QOTD,
  NTP,
  MEMCACHED,
  DNS,
  COAP,
  LDAP,
  CHARGEN,
}

impl PcapPacket {
  pub fn insert(
    &self,
    conn: &Connection,
    attack_id: &u32,
    tb_ip_id: &mut u32,
    hm_ip_id: &mut HashMap<String, u32>,
    vitima_cidr_id: &u32,
  ) {
    let mut result: Result<usize, rusqlite::Error> = Ok(0);

    self.ip.insert(conn, tb_ip_id, hm_ip_id, vitima_cidr_id);
    self.udp.insert(conn);

    match self.attack_type {
      PcapAttackType::DNS => match self.dns {
        None => {
          println!("Error inserting none dns")
        }
        Some(ref dns) => {
          dns.insert(conn);
          result = conn.execute(
                "INSERT INTO PCAP_PACKET (id, timestamp_str, attack_id, ip_id, udp_id, attack_type, dns_id) values ($1, $2, $3, $4, $5, $6)",
                params![
                  &self.id,
                  &self.timestamp.to_string(),
                  attack_id,
                  &self.ip.id,
                  &self.udp.id,
                  &self.attack_type.to_string(),
                  &dns.id
                ],
              );
        }
      },
      PcapAttackType::LDAP => match self.ldap {
        None => {
          println!("Error inserting none ldap")
        }
        Some(ref ldap) => {
          ldap.insert(conn);
          result = conn.execute(
                "INSERT INTO PCAP_PACKET (id, timestamp_str, ip_id, udp_id, attack_type, ldap_id) values ($1, $2, $3, $4, $5, $6)",
                params![&self.id, &self.timestamp.to_string(), &self.ip.id, &self.udp.id, &self.attack_type.to_string(), &ldap.id],
              );
        }
      },
      PcapAttackType::NTP => match self.ntp {
        None => {
          println!("Error inserting none ntp")
        }
        Some(ref ntp) => {
          ntp.insert(conn);
          result = conn.execute(
                "INSERT INTO PCAP_PACKET (id, timestamp_str, ip_id, udp_id, attack_type, ntp_id) values ($1, $2, $3, $4, $5, $6)",
                params![
                  &self.id,
                  &self.timestamp.to_string(),
                  &self.ip.id,
                  &self.udp.id,
                  &self.attack_type.to_string(),
                  &ntp.id
                ],
              );
        }
      },
      _ => {}
    }

    match result {
      Ok(_) => {
        //println!("INSERTED Packet")
      }
      Err(err) => {
        println!(
          "Problem inserting packet: {:?} - packet id: {}",
          err, self.id
        )
      }
    }
  }
}

impl PcapAttackType {
  pub fn to_string(&self) -> String {
    match self {
      PcapAttackType::STEAM => String::from("STEAM_GAMES"),
      PcapAttackType::SSDP => String::from("SSDP"),
      PcapAttackType::QOTD => String::from("QOTD"),
      PcapAttackType::NTP => String::from("NTP"),
      PcapAttackType::MEMCACHED => String::from("MEMCACHED"),
      PcapAttackType::DNS => String::from("DNS"),
      PcapAttackType::COAP => String::from("COAP"),
      PcapAttackType::LDAP => String::from("LDAP"),
      PcapAttackType::CHARGEN => String::from("CHARGEN"),
    }
  }
}

pub fn process_block(
  block: &PcapBlockOwned,
  hm_id: &mut HashMap<&str, u32>,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) -> Option<PcapPacket> {
  match block {
    LegacyHeader(_) => {
      // println!("PCAP Header size {}", header.size());
      return None;
    }
    Legacy(block) => {
      return process_legacy_block(block, hm_id, hm_ip_cidr);
    }
    NG(new_block) => {
      println!("NG new block in pcap {}", new_block.magic());
      return None;
    }
  }
}

fn process_legacy_block(
  b: &LegacyPcapBlock,
  hm_id: &mut HashMap<&str, u32>,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) -> Option<PcapPacket> {
  let naive_date_time = NaiveDateTime::from_timestamp(i64::from(b.ts_sec), b.ts_usec);

  match SlicedPacket::from_ethernet(b.data) {
    Ok(sliced_packet) => {
      return process_sliced_packet(sliced_packet, naive_date_time, hm_id, hm_ip_cidr);
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
  hm_id: &mut HashMap<&str, u32>,
  hm_ip_cidr: &mut HashMap<String, Ipv4Cidr>,
) -> Option<PcapPacket> {
  let ip = match sliced_packet.ip {
    Some(ip) => match ip {
      etherparse::InternetSlice::Ipv4(ipv4_header, _) => {
        let id_ipv4 = hm_id.entry("ipv4").or_insert(0);
        *id_ipv4 += 1;
        ip::process_ip(ipv4_header, *id_ipv4, hm_ip_cidr)
      }
      etherparse::InternetSlice::Ipv6(_, _) => {
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
        let id_udp = hm_id.entry("udp").or_insert(0);
        *id_udp += 1;
        udp::process_udp(udp_slice, *id_udp)
      }
      etherparse::TransportSlice::Icmpv4(_) => return None,
      etherparse::TransportSlice::Icmpv6(_) => return None,
      etherparse::TransportSlice::Tcp(_) => return None,
      etherparse::TransportSlice::Unknown(_) => return None,
    },
    None => {
      return None;
    }
  };

  // https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml?search=68&page=3
  let attack_type = match udp.destination_port {
    27015 => PcapAttackType::STEAM,
    1900 => PcapAttackType::SSDP,
    17 => PcapAttackType::QOTD,
    123 => PcapAttackType::NTP,
    11211 => PcapAttackType::MEMCACHED,
    53 => PcapAttackType::DNS,
    5683 => PcapAttackType::COAP,
    389 => PcapAttackType::LDAP, // LDAP CLDAP USES the same port
    19 => PcapAttackType::CHARGEN,
    _ => {
      // means that is none of those ports and then we do not need to use this packet
      return None;
    }
  };

  // só queremos pacotes em que o ip de destino é "200.19.107.238" "200.19.107.255"
  if !(ip.dest_addr.eq(&String::from("200.19.107.238"))
    || ip.dest_addr.eq(&String::from("200.19.107.255")))
  {
    // println!("DESCARTAR: UDP dest port: {} ip source: {} ip dest: {}", udp.destination_port, ip.vitima_addr, ip.dest_addr);
    return None;
  }
  // println!("processando: UDP dest port: {} ip source: {} ip dest: {}", udp.destination_port, ip.vitima_addr, ip.dest_addr);

  let id_packet = hm_id.entry("packet").or_insert(0);
  *id_packet += 1;
  let mut packet = PcapPacket {
    id: *id_packet,
    ip,
    udp,
    timestamp,
    attack_type,
    ntp: None,
    dns: None,
    ldap: None,
  };

  match packet.attack_type {
    PcapAttackType::NTP => match ntp_parser::parse_ntp(sliced_packet.payload) {
      Ok((_, ref ntp_packet)) => {
        let id_ntp = hm_id.entry("ntp").or_insert(0);
        *id_ntp += 1;
        packet.ntp = Some(ntp::process_ntp(ntp_packet, *id_ntp));
        packet.attack_type = PcapAttackType::NTP;
        return Some(packet);
      }
      Err(_) => {
        return None;
      }
    },
    PcapAttackType::DNS => match dns_parser::Packet::parse(sliced_packet.payload) {
      Ok(ref dns_packet) => {
        let id_dns = hm_id.entry("dns").or_insert(0);
        *id_dns += 1;
        packet.dns = Some(dns::process_dns(dns_packet, *id_dns));
        packet.attack_type = PcapAttackType::DNS;
        return Some(packet);
      }
      Err(_) => {
        return None;
      }
    },
    PcapAttackType::LDAP => {
      match ldap_parser::parse_ldap_messages(sliced_packet.payload) {
        Ok((_, ref ldap)) => {
          for msg in ldap {
            let id_ldap = hm_id.entry("ldap").or_insert(0);
            *id_ldap += 1;
            packet.ldap = Some(ldap::process_ldap(msg, *id_ldap));
            packet.attack_type = PcapAttackType::LDAP;
            // we only care for the first msg
            return Some(packet);
          }
        }
        Err(_) => {
          // return None;
        }
      }
    }
    _ => {}
  }

  return Some(packet);
}
