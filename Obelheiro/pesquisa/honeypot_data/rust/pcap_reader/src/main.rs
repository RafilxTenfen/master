use chrono::{DateTime, Duration, FixedOffset};
use cidr_utils::cidr::Ipv4Cidr;
use rtshark::{RTShark, RTSharkBuilder};
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
// use std::time::Duration;

mod pcap_packet;
// use std::path::Path;

fn main() -> Result<()> {
  let currently_dir = get_current_working_dir();
  println!("Working dir {}", currently_dir.display());

  let db_path = currently_dir
    .as_path()
    .join("../../db/database-2022-05-11/mix_protocol.sqlite");
  let conn = Connection::open(db_path)?;
  drop_tables(&conn);
  create_tables(&conn);

  // loop entre vários arquivos pcaps, ordenados pela data '-'
  let pcap_str = currently_dir
    .as_path()
    .join("../../pcap/tcpdump.out.2021-04-14_14-55-38.377363_dcn.pcap00000000")
    .into_os_string()
    .into_string()
    .unwrap();
  println!("PCAP dir {}", pcap_str);

  // HashMap CIDR => UDP dest port => Attack
  let mut map_attacks = HashMap::<Ipv4Cidr, HashMap<i32, PcapAttack>>::new();
  let mut map_id = HashMap::<&str, i32>::new();
  let mut id_attack = 0;

  let builder = RTSharkBuilder::builder().input_path(&pcap_str);
  // builder.metadata_blacklist(blacklist)
  // builder.metadata_blacklist(blacklist)
  // Start a new TShark process
  let rtshark = builder
    .spawn()
    .unwrap_or_else(|e| panic!("Error starting tshark: {e}"));

  pcap_process(
    rtshark,
    &conn,
    &mut map_id,
    &mut map_attacks,
    &mut id_attack,
  );

  Ok(())
}

pub struct PcapAttack {
  id: i32,
  ip_vitima_cidr: Ipv4Cidr,
  packets: Vec<pcap_packet::PcapPacket>,
  timestamp_inicio: DateTime<FixedOffset>,
  timestamp_fim: DateTime<FixedOffset>,
}

impl PcapAttack {
  pub fn add_packet(&mut self, packet: pcap_packet::PcapPacket) {
    self.timestamp_fim = packet.frame.timestamp;
    self.packets.push(packet);
  }

  // same_attack chcks if the packet happened within 5 minutes from the last attack
  pub fn same_attack(&self, packet: &pcap_packet::PcapPacket) -> bool {
    match self.timestamp_fim.checked_add_signed(Duration::minutes(1)) {
      Some(fim_plus_1min) => {
        packet.frame.timestamp <= fim_plus_1min && packet.frame.timestamp >= self.timestamp_fim
      }
      None => false,
    }
  }

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_ATTACK (id, ip_vitima_cidr, packets_per_attack, timestamp_inicio, timestamp_fim) values (?1, ?2, ?3, ?4)",
      params![&self.id, &self.ip_vitima_cidr.to_string(), &self.packets.len(), &self.timestamp_inicio.to_string(), &self.timestamp_fim.to_string()],
  ) {
      Ok(_) => {
        println!("attack inserted");
        self.insert_pcap_packets(conn);
      }
      Err(err) => {
        println!("Problem inserting attack: {:?}", err)
      }
    }
  }

  fn insert_pcap_packets(&self, conn: &Connection) {
    for packet in &self.packets {
      match conn.execute(
        "INSERT INTO PCAP_ATTACK_PACKET (attack_id, packet_id) values (?1, ?2)",
        params![&self.id, &packet.id],
      ) {
        Ok(_) => {
          println!("attack inserted")
        }
        Err(err) => {
          println!("Problem inserting attack : {:?}", err)
        }
      }
    }
  }
}

fn get_current_working_dir() -> PathBuf {
  let working_dir = env::current_dir();
  let current_dir = match working_dir {
    Ok(workdir) => workdir,
    Err(error) => panic!("Problem reading current dir: {:?}", error),
  };
  return current_dir;
}

pub fn pcap_process(
  mut rtshark: RTShark,
  conn: &Connection,
  map_id: &mut HashMap<&str, i32>,
  map_attacks: &mut HashMap<Ipv4Cidr, HashMap<i32, PcapAttack>>,
  id_attack: &mut i32,
) {
  // read packets until the end of the PCAP file

  while let Some(packet) = rtshark.read().unwrap_or_else(|e| {
    eprintln!("Error parsing TShark output: {e}");
    None
  }) {
    // packet.into()
    // 5 pacotes
    // intervalo de 1 minutos
    // source IP (ip.src - vítima) do mesmo CIDR block e mesma porta destino UDP
    // cidr utils

    // for layer in packet {
    //   println!("Layer name: {}", layer.name());
    //   for metadata in layer {
    //     println!("\t metadata Name: {} = {}", metadata.name(), metadata.display());
    //   }
    // }

    let packet = pcap_packet::pcap_process_packet(&packet, map_id);
    add_packet_to_attacks(conn, map_attacks, packet, id_attack);

    // TODO verificar a cada 10000 packets o que da pra limpar do hashmap...
  }
}

pub fn add_packet_to_attacks(
  conn: &Connection,
  cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i32, PcapAttack>>,
  packet: pcap_packet::PcapPacket,
  id_attack: &mut i32,
) {
  let cidr = packet.ip.vitima_cidr;

  // cidr => udp => attack
  match cidr_udp_attack.get_mut(&cidr) {
    Some(udp_attack) => {
      // cidr existe
      // udp => attack
      let udp_dest_port = packet.udp.destination_port;

      match udp_attack.get_mut(&udp_dest_port) {
        Some(attack) => {
          // attack exists
          // check time
          if attack.same_attack(&packet) {
            attack.add_packet(packet);
            return;
          }

          // not same attack, should check if len(packets) > 5 to dbinsert or just replace by a new attack
          if attack.packets.len() > 5 {
            attack.insert(conn);
          }
          udp_attack.insert(udp_dest_port, new_empty_attack(packet, id_attack));
          return;
        }
        None => {
          // no attack
          udp_attack.insert(udp_dest_port, new_empty_attack(packet, id_attack));
          return;
        }
      }
    }

    None => {
      // não tem o cidr,
      let udp_attack = new_map_attack(packet, id_attack);
      cidr_udp_attack.insert(cidr, udp_attack);
      return;
    }
  }
}

pub fn new_map_attack(
  packet: pcap_packet::PcapPacket,
  id_attack: &mut i32,
) -> HashMap<i32, PcapAttack> {
  let mut map_attack = HashMap::<i32, PcapAttack>::new();
  map_attack.insert(
    packet.udp.destination_port,
    new_empty_attack(packet, id_attack),
  );

  return map_attack;
}

pub fn new_empty_attack(packet: pcap_packet::PcapPacket, id: &mut i32) -> PcapAttack {
  let timestamp = packet.frame.timestamp;
  *id += 1;

  return PcapAttack {
    id: *id,
    ip_vitima_cidr: packet.ip.vitima_cidr,
    packets: vec![packet],
    timestamp_inicio: timestamp,
    timestamp_fim: timestamp,
  };
}

pub fn drop_tables(conn: &Connection) {
  pcap_packet::db_drop_pcap_tables(&conn);
  drop_table_attack(conn);
  pcap_packet::db_create_pcap_tables(&conn);
}

pub fn create_tables(conn: &Connection) {
  pcap_packet::db_create_pcap_tables(&conn);
  create_table_attack(conn);
}

pub fn drop_table_attack(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_ATTACK", []) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem droping table PCAP_ATTACK: {:?}", err)
    }
  }

  match conn.execute("DROP TABLE IF EXISTS PCAP_ATTACK_PACKET", []) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem droping table PCAP_ATTACK_PACKET: {:?}", err)
    }
  }
}

pub fn create_table_attack(conn: &Connection) {
  match conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_ATTACK (
      id INTEGER NOT NULL,
      ip_vitima_cidr TEXT NOT NULL,
      packets_per_attack INTEGER NOT NULL,
      timestamp_inicio TEXT NOT NULL,
      timestamp_fim TEXT NOT NULL
     )",
    [],
  ) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem creating table ATTACK: {:?}", err)
    }
  }

  // FOREIGN KEY(attack_id) REFERENCES PCAP_ATTACK(id),
  // FOREIGN KEY(packet_id) REFERENCES PCAP_PACKET(id)

  match conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_ATTACK_PACKET (
      attack_id INTEGER NOT NULL,
      packet_id INTEGER NOT NULL
    )",
    [],
  ) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem creating table PCAP_ATTACK_PACKET: {:?}", err)
    }
  }
}
