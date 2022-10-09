use chrono::Duration;
use chrono::NaiveDateTime;
use cidr_utils::cidr::Ipv4Cidr;
use postgres::Transaction;
// use rusqlite::params;
// use rusqlite::Connection;
use std::collections::HashMap;

use super::block::PcapPacket;

pub struct PcapAttack {
  pub id: i32,
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

  pub fn new_attack(packet: PcapPacket, id: &mut i32) -> PcapAttack {
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

  pub fn insert(
    &self,
    conn: &mut Transaction,
    hm_id: &mut HashMap<&str, i32>,
    tb_ip_id: &mut i32,
    hm_ip_id: &mut HashMap<String, i32>,
    hm_cidr_id: &mut HashMap<Ipv4Cidr, i32>,
  ) {
    let packet_len = self.packets.len() as i32;

    let vitima_cidr_id = match hm_cidr_id.get(&self.ip_vitima_cidr) {
      Some(id_cidr) => id_cidr,
      None => {
        let cidr_id = hm_id.entry("tbcidr").or_insert(0);
        *cidr_id += 1;
        match conn.execute(
          "INSERT INTO TBCIDR (id, cidr) values ($1, $2)",
          &[cidr_id, &self.ip_vitima_cidr.to_string()],
        ) {
          Ok(_) => {}
          Err(err) => {
            println!("Problem inserting TBCIDR: {:?}", err)
          }
        }
        hm_cidr_id.insert(self.ip_vitima_cidr, *cidr_id);
        cidr_id
      }
    };

    match conn.execute(
      "INSERT INTO PCAP_ATTACK (id, vitima_cidr_id, packets_per_attack, timestamp_inicio, timestamp_fim) values ($1, $2, $3, $4, $5)",
      &[&self.id, vitima_cidr_id, &packet_len, &self.timestamp_inicio.to_string(), &self.timestamp_fim.to_string()],
    ) {
      Ok(_) => {
        // println!("attack inserted");
        self.insert_pcap_packets(conn, tb_ip_id, hm_ip_id, vitima_cidr_id);
      }
      Err(err) => {
        println!("Problem inserting attack: {:?}", err)
      }
    }
  }

  fn insert_pcap_packets(
    &self,
    conn: &mut Transaction,
    tb_ip_id: &mut i32,
    hm_ip_id: &mut HashMap<String, i32>,
    vitima_cidr_id: &i32,
  ) {
    for packet in &self.packets {
      packet.insert(conn, &self.id, tb_ip_id, hm_ip_id, vitima_cidr_id);
    }
  }
}

pub fn new_hm_udp_attack(packet: PcapPacket, id_attack: &mut i32) -> HashMap<i16, PcapAttack> {
  let mut map_attack = HashMap::<i16, PcapAttack>::new();
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
  conn: &mut Transaction,
  hm_cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i16, PcapAttack>>,
  hm_id: &mut HashMap<&str, i32>,
  tb_ip_id: &mut i32,
  hm_ip_id: &mut HashMap<String, i32>,
  hm_cidr_id: &mut HashMap<Ipv4Cidr, i32>,
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
            attack.insert(conn, hm_id, tb_ip_id, hm_ip_id, hm_cidr_id); // inserts db
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
