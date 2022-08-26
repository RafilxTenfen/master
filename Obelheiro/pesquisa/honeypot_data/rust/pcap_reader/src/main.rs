use rtshark::{RTShark, RTSharkBuilder};
use rusqlite::{Connection, Result};
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;
use cidr_utils::cidr::Ipv4Cidr;
use chrono::{DateTime, FixedOffset};

mod pcap_packet;
// use std::path::Path;

pub struct PcapAttack {
  id: i32,
  ip_vitima_cidr: Ipv4Cidr,
  packets: Vec<pcap_packet::PcapPacket>,
  timestamp_inicio: DateTime<FixedOffset>,
  timestamp_fim: DateTime<FixedOffset>,
}

fn get_current_working_dir() -> PathBuf {
  let working_dir = env::current_dir();
  let current_dir = match working_dir {
    Ok(workdir) => workdir,
    Err(error) => panic!("Problem reading current dir: {:?}", error),
  };
  return current_dir;
}

fn main() -> Result<()> {
  let currently_dir = get_current_working_dir();
  println!("Working dir {}", currently_dir.display());

  let db_path = currently_dir
    .as_path()
    .join("../../db/database-2022-05-11/mix_protocol.sqlite");
  let conn = Connection::open(db_path)?;
  pcap_packet::db_drop_pcap_tables(&conn);
  pcap_packet::db_create_pcap_tables(&conn);

  let pcap_str = currently_dir
    .as_path()
    .join("../../pcap/tcpdump.out.2021-04-14_14-55-38.377363_dcn.pcap00000000")
    .into_os_string()
    .into_string()
    .unwrap();
  println!("PCAP dir {}", pcap_str);

  let mut attack_id = 1;
  // HashMap CIDR => UDP dest port => Attack
  let mut map_attacks = HashMap::<Ipv4Cidr, HashMap<i32, PcapAttack>>::new();
  let mut map_id = HashMap::<&str, i32>::new();

  let builder = RTSharkBuilder::builder().input_path(&pcap_str);
  // Start a new TShark process
  let rtshark = builder
    .spawn()
    .unwrap_or_else(|e| panic!("Error starting tshark: {e}"));

  pcap_process(rtshark, &conn, &mut map_id, &mut map_attacks, &mut attack_id);

  Ok(())
}

pub fn pcap_process(
  mut rtshark: RTShark,
  conn: &Connection,
  map_id: &mut HashMap<&str, i32>,
  map_attacks: &mut HashMap<Ipv4Cidr, HashMap<i32, PcapAttack>>,
  attack_id: &mut i32,
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

      // println!("Layer name: {}", layer.name());
      // for metadata in layer {
      //   println!("\t metadata Name: {} = {}", metadata.name(), metadata.display());
      // }

    let packet = pcap_packet::pcap_process_packet(packet, conn, map_id);
    add_packet_to_attacks(map_attacks, packet, *attack_id);

    *attack_id += 1;
  }
}

pub fn add_packet_to_attacks(cidr_udp_attack: &mut HashMap<Ipv4Cidr, HashMap<i32, PcapAttack>>, packet: pcap_packet::PcapPacket, attack_id: i32) {
  let cidr = packet.ip.vitima_cidr;

  match cidr_udp_attack.get_mut(&cidr) {
    Some(udp_attack) => {
      // cidr existe
      let udp_dest_port = packet.udp.destination_port;
      udp_attack.insert(udp_dest_port, new_attack(packet, attack_id));
    },
    None => {
      // não tem o cidr,
      let udp_attack = new_map_attack(packet, attack_id);
      cidr_udp_attack.insert(cidr, udp_attack);
    },
  }


  // if !udp_attack.contains_key(&udp_dest_port) {
  //   // tem o cidr
  //   // mas nao tem a porta de destino udp
  //   // let attack = new_attack(packet, attack_id);
  //   // udp_attack.insert(udp_dest_port, attack);
  //   return ;
  // }
}

pub fn new_map_attack(packet: pcap_packet::PcapPacket, attack_id: i32) -> HashMap<i32, PcapAttack> {
  let mut map_attack = HashMap::<i32, PcapAttack>::new();
  map_attack.insert(packet.udp.destination_port, new_attack(packet, attack_id));

  return map_attack;
}

pub fn new_attack(packet: pcap_packet::PcapPacket, attack_id: i32) -> PcapAttack {
  let timestamp = packet.frame.timestamp;

  return PcapAttack{
    id: attack_id,
    ip_vitima_cidr: packet.ip.vitima_cidr,
    packets: vec![packet],
    timestamp_inicio: timestamp,
    timestamp_fim: timestamp,
  }
}