use rtshark::{Layer, Packet};
use rusqlite::{params, Connection};
use std::collections::HashMap;
mod pcap_chargen;
mod pcap_dns;
mod pcap_frame;
mod pcap_ip;
mod pcap_ntp;
mod pcap_ssdp;
mod pcap_udp;

pub struct PcapPacket {
  pub id: i32,
  pub frame: pcap_frame::PcapFrame,
  pub ip: pcap_ip::PcapIP,
  pub udp: pcap_udp::PcapUDP,
  pub attack_type: PcapAttackType,
  pub dns: Option<pcap_dns::PcapDNS>,
  pub chargen: Option<pcap_chargen::PcapChargen>,
  pub ntp: Option<pcap_ntp::PcapNTP>,
  pub ssdp: Option<pcap_ssdp::PcapSSDP>,
}

pub enum PcapAttackType {
  None,
  DNS,
  CHARGEN,
  NTP,
  SSDP,
}

impl PcapAttackType {
  pub fn to_string(&self) -> String {
    match self {
      PcapAttackType::None => String::from("None"),
      PcapAttackType::DNS => String::from("DNS"),
      PcapAttackType::CHARGEN => String::from("CHARGEN"),
      PcapAttackType::NTP => String::from("NTP"),
      PcapAttackType::SSDP => String::from("SSDP"),
    }
  }
}

pub fn db_create_pcap_tables(conn: &Connection) {
  pcap_dns::create_table(conn);
  pcap_frame::create_table(conn);
  pcap_ip::create_table(conn);
  pcap_udp::create_table(conn);
  pcap_ntp::create_table(conn);
  pcap_chargen::create_table(conn);
  pcap_ssdp::create_table(conn);
  create_table(conn);
}

pub fn db_drop_pcap_tables(conn: &Connection) {
  drop_table(conn);
  pcap_dns::drop_table(conn);
  pcap_frame::drop_table(conn);
  pcap_ip::drop_table(conn);
  pcap_udp::drop_table(conn);
  pcap_ntp::drop_table(conn);
  pcap_chargen::drop_table(conn);
  pcap_ssdp::drop_table(conn);
}

pub fn drop_table(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_PACKET", []) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem droping table: {:?}", err)
    }
  }
}

pub fn create_table(conn: &Connection) {
  // FOREIGN KEY(frame_id) REFERENCES PCAP_FRAME(id),
  // FOREIGN KEY(ip_id) REFERENCES PCAP_IP(id),
  // FOREIGN KEY(udp_id) REFERENCES PCAP_UDP(id),
  // FOREIGN KEY(dns_id) REFERENCES PCAP_DNS(id),
  // FOREIGN KEY(chargen_id) REFERENCES PCAP_CHARGEN(id),
  // FOREIGN KEY(ntp_id) REFERENCES PCAP_NTP(id),
  // FOREIGN KEY(ssdp_id) REFERENCES PCAP_SSDP(id)

  match conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_PACKET (
      id INTEGER NOT NULL,
      frame_id INTEGER NOT NULL,
      ip_id INTEGER NOT NULL,
      udp_id INTEGER NOT NULL,
      attack_type TEXT NOT NULL,
      dns_id INTEGER,
      chargen_id INTEGER,
      ntp_id INTEGER,
      ssdp_id INTEGER
    )",
    [],
  ) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem creating table: {:?}", err)
    }
  }
}

impl std::fmt::Display for PcapPacket {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "(
      id.to_string(): {},
      frame.id.to_string(): {},
      ip.id.to_string(): {},
      udp.id.to_string(): {},
    )",
      self.id.to_string(),
      self.frame.id.to_string(),
      self.ip.id.to_string(),
      self.udp.id.to_string(),
    )
  }
}

impl PcapPacket {
  pub fn default(id: i32) -> PcapPacket {
    PcapPacket {
      id,
      frame: pcap_frame::PcapFrame::default(0),
      ip: pcap_ip::PcapIP::default(0),
      udp: pcap_udp::PcapUDP::default(0),
      attack_type: PcapAttackType::None,
      dns: None,
      chargen: None,
      ntp: None,
      ssdp: None,
    }
  }

  pub fn process_layer(&mut self, layer: &Layer, map_id: &mut HashMap<&str, i32>) {
    let layer_name = layer.name();

    match layer_name {
      "frame" => {
        let id = map_id.entry("frame").or_insert(0);
        self.frame = pcap_frame::pcap_process_layer_frame(layer, id);
        *id += 1;
      }
      "ip" => {
        let id = map_id.entry("ip").or_insert(0);
        self.ip = pcap_ip::pcap_process_layer_ip(layer, id);
        *id += 1;
      }
      "udp" => {
        let id = map_id.entry("udp").or_insert(0);
        self.udp = pcap_udp::pcap_process_layer_udp(layer, id);
        *id += 1;
      }
      "dns" => {
        let id = map_id.entry("dns").or_insert(0);
        self.attack_type = PcapAttackType::DNS;
        self.dns = Some(pcap_dns::pcap_process_layer_dns(layer, *id));
        *id += 1;
      }
      "ntp" => {
        let id = map_id.entry("ntp").or_insert(0);
        self.attack_type = PcapAttackType::NTP;
        self.ntp = Some(pcap_ntp::pcap_process_layer_ntp(layer, id));
        *id += 1;
      }
      "chargen" => {
        let id = map_id.entry("chargen").or_insert(0);
        self.attack_type = PcapAttackType::CHARGEN;
        self.chargen = Some(pcap_chargen::pcap_process_layer_chargen(layer, *id));
        *id += 1;
      }
      "ssdp" => {
        let id = map_id.entry("ssdp").or_insert(0);
        self.attack_type = PcapAttackType::SSDP;
        self.ssdp = Some(pcap_ssdp::pcap_process_layer_ssdp(layer, id));
        *id += 1;
      }
      "eth" => {
        println!("layer is eth - we don't want nothing here")
      }
      "tcp" => {
        println!("layer is tcp - we don't want nothing here")
      }
      _ => {
        println!("layer is {}", layer_name);
        // let it = layer.iter();
        // for metadata in layer {
        //   println!(
        //     "\t metadata Name: {} = {} - {}",
        //     metadata.name(),
        //     metadata.value(),
        //     metadata.display(),
        //   );
        // }
      }
    }
    // *layer_id += 1
  }

  pub fn insert(&self, conn: &Connection) {
    let mut result: Result<usize, rusqlite::Error> = Ok(0);

    match self.attack_type {
      PcapAttackType::None => {}
      PcapAttackType::DNS => match &self.dns {
        None => {
          println!("Error inserting none dns")
        }
        Some(dns) => {
          dns.insert(conn);
          result = conn.execute(
                "INSERT INTO PCAP_PACKET (id, frame_id, ip_id, udp_id, dns_id) values (?1, ?2, ?3, ?4, ?5)",
                params![
                  &self.id,
                  &self.frame.id,
                  &self.ip.id,
                  &self.udp.id,
                  &dns.id
                ],
              );
        }
      },
      PcapAttackType::CHARGEN => match &self.chargen {
        None => {
          println!("Error inserting none chargen")
        }
        Some(chargen) => {
          chargen.insert(conn);
          result = conn.execute(
                "INSERT INTO PCAP_PACKET (id, frame_id, ip_id, udp_id, chargen_id) values (?1, ?2, ?3, ?4, ?5)",
                params![&self.id, &self.frame.id, &self.ip.id, &self.udp.id, &chargen.id],
              );
        }
      },
      PcapAttackType::NTP => match &self.ntp {
        None => {
          println!("Error inserting none ntp")
        }
        Some(ntp) => {
          ntp.insert(conn);
          result = conn.execute(
                "INSERT INTO PCAP_PACKET (id, frame_id, ip_id, udp_id, ntp_id) values (?1, ?2, ?3, ?4, ?5)",
                params![
                  &self.id,
                  &self.frame.id,
                  &self.ip.id,
                  &self.udp.id,
                  &ntp.id
                ],
              );
        }
      },
      PcapAttackType::SSDP => match &self.ssdp {
        None => {
          println!("Error inserting none ssdp")
        }
        Some(ssdp) => {
          ssdp.insert(conn);
          result = conn.execute(
                "INSERT INTO PCAP_PACKET (id, frame_id, ip_id, udp_id, ssdp_id) values (?1, ?2, ?3, ?4, ?5)",
                params![&self.id, &self.frame.id, &self.ip.id, &self.udp.id, &ssdp.id],
              );
        }
      },
    }

    match result {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting packet: {:?} = {}", err, self)
      }
    }
  }
}

pub fn pcap_process_packet(packet: &Packet, map_id: &mut HashMap<&str, i32>) -> PcapPacket {
  let packet_entry = map_id.entry("packet");
  let packet_id = packet_entry.or_insert(0);

  *packet_id += 1;

  let mut pcap_packet = PcapPacket::default(*packet_id);

  println!("Packet {}", packet_id);

  packet
    .iter()
    .for_each(|layer| pcap_packet.process_layer(layer, map_id));

  return pcap_packet;
}
