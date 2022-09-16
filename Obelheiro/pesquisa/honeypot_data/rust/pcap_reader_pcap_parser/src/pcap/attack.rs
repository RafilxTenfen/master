use chrono::Duration;
use chrono::NaiveDateTime;
use cidr_utils::cidr::Ipv4Cidr;
use rusqlite::Connection;
use rusqlite::{params};
use std::collections::HashMap;

use super::block::PcapPacket;

pub struct PcapAttack {
  pub id: u32,
  pub ip_vitima_cidr: Ipv4Cidr,
  pub packets: Vec<PcapPacket>,
  pub timestamp_inicio: NaiveDateTime,
  pub timestamp_fim: NaiveDateTime,
}

impl PcapAttack {
  pub fn add_packet(&mut self, packet: PcapPacket) {
    self.timestamp_fim = packet.timestamp;
    self.packets.push(packet);
  }

  // same_attack chcks if the packet happened within 1 minute from the last attack
  pub fn same_attack(&self, packet: &PcapPacket) -> bool {
    match self.timestamp_fim.checked_add_signed(Duration::minutes(1)) {
      Some(fim_plus_1min) => {
        return packet.timestamp >= self.timestamp_fim && packet.timestamp <= fim_plus_1min
      }
      None => false,
    }
  }

  pub fn new_attack(packet: PcapPacket, id: &mut u32) -> PcapAttack {
    let timestamp = packet.timestamp;
    *id += 1;

    return PcapAttack {
      id: *id,
      ip_vitima_cidr: packet.ip.vitima_cidr,
      packets: vec![packet],
      timestamp_inicio: timestamp,
      timestamp_fim: timestamp,
    };
  }

  // DATABASE FUNCTIONS

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_ATTACK (id, ip_vitima_cidr, packets_per_attack, timestamp_inicio, timestamp_fim) values (?1, ?2, ?3, ?4, ?5)",
      params![&self.id, &self.ip_vitima_cidr.to_string(), &self.packets.len(), &self.timestamp_inicio.to_string(), &self.timestamp_fim.to_string()],
    ) {
      Ok(_) => {
        // println!("attack inserted");
        self.insert_pcap_packets(conn);
      }
      Err(err) => {
        println!("Problem inserting attack: {:?}", err)
      }
    }
  }

  fn insert_pcap_packets(&self, conn: &Connection) {
    for packet in &self.packets {
      packet.insert(conn);
      match conn.execute(
        "INSERT INTO PCAP_ATTACK_PACKET (attack_id, packet_id) values (?1, ?2)",
        params![&self.id, &packet.id],
      ) {
        Ok(_) => {
          // println!("attack inserted")
        }
        Err(err) => {
          println!("Problem inserting attack_packet : {:?}", err)
        }
      }
    }
  }
}

pub fn new_hm_udp_attack(packet: PcapPacket, id_attack: &mut u32) -> HashMap<u16, PcapAttack> {
  let mut map_attack = HashMap::<u16, PcapAttack>::new();
  map_attack.insert(
    packet.udp.destination_port,
    PcapAttack::new_attack(packet, id_attack),
  );

  return map_attack;
}

// 5 pacotes
// intervalo de 1 minutos
// source IP (ip.src - vítima) do mesmo CIDR block e mesma porta destino UDP
pub fn process_new_packet(
  conn: &Connection,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<u16, PcapAttack>>,
  hm_id: &mut HashMap<&str, u32>,
  new_packet: PcapPacket,
) {
  let cidr = new_packet.ip.vitima_cidr;

  // cidr => udp => attack
  match hm_cidr_udp_attack.get_mut(&cidr) {
    Some(udp_attack) => {
      // cidr existe
      // udp => attack
      let udp_dest_port = new_packet.udp.destination_port;

      match udp_attack.get_mut(&udp_dest_port) {
        Some(attack) => {
          // attack exists
          // check time
          if attack.same_attack(&new_packet) {
            attack.add_packet(new_packet);
            return;
          }

          // not same attack, timestamp passed
          // should check if len(packets) > 5 to dbinsert or just replace by a new attack
          if attack.packets.len() > 5 {
            // println!(
            //   "process_new_packet id: {} - len packets: {}",
            //   attack.id,
            //   attack.packets.len()
            // );
            attack.insert(conn); // inserts db
          }

          let id_attack = hm_id.entry("attack").or_insert(0);
          // udp_attack.insert also updates
          udp_attack.insert(udp_dest_port, PcapAttack::new_attack(new_packet, id_attack));
          return;
        }
        None => {
          let id_attack = hm_id.entry("attack").or_insert(0);
          // no attack
          udp_attack.insert(udp_dest_port, PcapAttack::new_attack(new_packet, id_attack));
          return;
        }
      }
    }

    None => {
      let id_attack = hm_id.entry("attack").or_insert(0);
      // não tem o cidr,
      let hm_udp_attack = new_hm_udp_attack(new_packet, id_attack);
      hm_cidr_udp_attack.insert(cidr, hm_udp_attack);
      return;
    }
  }
}
