use rtshark::Packet;
use rusqlite::{Connection, params};
use std::collections::HashMap;
mod pcap_dns;
mod pcap_frame;
mod pcap_ip;
mod pcap_udp;
mod pcap_ntp;
mod pcap_chargen;
mod pcap_ssdp;


pub struct PcapPacket {
  pub id: i32,
  pub frame: pcap_frame::PcapFrame,
  pub ip: pcap_ip::PcapIP,
  pub udp: pcap_udp::PcapUDP,
  pub dns: pcap_dns::PcapDNS,
  pub chargen: pcap_chargen::PcapChargen,
  pub ntp: pcap_ntp::PcapNTP,
  pub ssdp: pcap_ssdp::PcapSSDP,
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
  let result = conn.execute(
    "DROP TABLE IF EXISTS PCAP_PACKET",
    [],
  );

  match result {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem droping table: {:?}", err)
    }
  }
}

pub fn create_table(conn: &Connection) {
  let result = conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_PACKET (
      id INTEGER NOT NULL,
      frame_id INTEGER NOT NULL,
      ip_id INTEGER NOT NULL,
      udp_id INTEGER NOT NULL,
      dns_id INTEGER,
      chargen_id INTEGER,
      ntp_id INTEGER,
      ssdp_id INTEGER
    )",
    [],
  );
  // FOREIGN KEY(frame_id) REFERENCES PCAP_FRAME(id),
  // FOREIGN KEY(ip_id) REFERENCES PCAP_IP(id),
  // FOREIGN KEY(udp_id) REFERENCES PCAP_UDP(id),
  // FOREIGN KEY(dns_id) REFERENCES PCAP_DNS(id),
  // FOREIGN KEY(chargen_id) REFERENCES PCAP_CHARGEN(id),
  // FOREIGN KEY(ntp_id) REFERENCES PCAP_NTP(id),
  // FOREIGN KEY(ssdp_id) REFERENCES PCAP_SSDP(id)

  match result {
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
    write!(f, "(
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
      dns: pcap_dns::PcapDNS::default(0),
      chargen: pcap_chargen::PcapChargen::default(0),
      ntp: pcap_ntp::PcapNTP::default(0),
      ssdp: pcap_ssdp::PcapSSDP::default(0),
      // dns: None,
      // chargen: None,
      // ntp: None,
      // ssdp: None,
    }
  }

  // pub fn get_dns_id(&self) -> String {
  //   if self.dns.id == 0 {
  //     return String::from("null");
  //   }

  //   return self.dns.id.to_string();
  // }

  // pub fn get_chargen_id(&self) -> String {
  //   if self.chargen.id == 0 {
  //     return String::from("null");
  //   }

  //   return self.chargen.id.to_string();
  // }

  // pub fn get_ntp_id(&self) -> String {
  //   if self.ntp.id == 0 {
  //     return String::from("null");
  //   }

  //   return self.ntp.id.to_string();
  // }

  // pub fn get_ssdp_id(&self) -> String {
  //   if self.ssdp.id == 0 {
  //     return String::from("null");
  //   }

  //   return self.ssdp.id.to_string();
  // }

  pub fn _insert(&self, conn: &Connection) {
    let mut result: Result<usize, rusqlite::Error> = Ok(0);

    if self.dns.id != 0 {
      result = conn.execute(
        "INSERT INTO PCAP_PACKET (id, frame_id, ip_id, udp_id, dns_id) values (?1, ?2, ?3, ?4, ?5)",
        params![&self.id, &self.frame.id, &self.ip.id, &self.udp.id, &self.dns.id],
      );
    } else if self.chargen.id != 0 {
      result = conn.execute(
        "INSERT INTO PCAP_PACKET (id, frame_id, ip_id, udp_id, chargen_id) values (?1, ?2, ?3, ?4, ?5)",
        params![&self.id, &self.frame.id, &self.ip.id, &self.udp.id, &self.chargen.id],
      );
    } else if self.ssdp.id != 0 {
      result = conn.execute(
        "INSERT INTO PCAP_PACKET (id, frame_id, ip_id, udp_id, ssdp_id) values (?1, ?2, ?3, ?4, ?5)",
        params![&self.id, &self.frame.id, &self.ip.id, &self.udp.id, &self.ssdp.id],
      );
    } else if self.ntp.id != 0 {
      result = conn.execute(
        "INSERT INTO PCAP_PACKET (id, frame_id, ip_id, udp_id, ntp_id) values (?1, ?2, ?3, ?4, ?5)",
        params![&self.id, &self.frame.id, &self.ip.id, &self.udp.id, &self.ntp.id],
      );
    }

    match result {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting packet: {:?} = {}", err, self)
      }
    }
  }
}

pub fn pcap_process_packet(packet: Packet, conn: &Connection, map_id: &mut HashMap<&str, i32>) -> PcapPacket {
  let packet_entry = map_id.entry("packet");
  let packet_id = packet_entry.or_insert(0);

  *packet_id += 1;

  let mut pcap_packet = PcapPacket::default(*packet_id);


  // map_id.into_iter();

  println!("Packet {}", packet_id);
  for layer in packet {
    let layer_name = layer.name();
    // let layer_name2 = layer.name();
    // let name_clone = layer_name.clone();
    // NAO CONSEGUI PEGAR O LAYER_NAME WTF RUST

    // let mut id = map_id.entry(layer_name.clone()).or_insert(0);



    match layer_name {
      "frame" => {
        let id = map_id.entry("frame").or_insert(0);
        pcap_packet.frame = pcap_frame::pcap_process_layer_frame(layer, conn, *id);
        *id += 1;
      },
      "ip" => {
        let id = map_id.entry("ip").or_insert(0);
        pcap_packet.ip = pcap_ip::pcap_process_layer_ip(layer, conn, *id);
        *id += 1;
      },
      "udp" => {
        let id = map_id.entry("udp").or_insert(0);
        pcap_packet.udp = pcap_udp::pcap_process_layer_udp(layer, conn, *id);
        *id += 1;
      },
      "dns" => {
        let id = map_id.entry("dns").or_insert(0);
        pcap_packet.dns = pcap_dns::pcap_process_layer_dns(layer, conn, *id);
        *id += 1;
      },
      "ntp" => {
        let id = map_id.entry("ntp").or_insert(0);
        pcap_packet.ntp = pcap_ntp::pcap_process_layer_ntp(layer, conn, *id);
        *id += 1;
      },
      "chargen" => {
        let id = map_id.entry("chargen").or_insert(0);
        pcap_packet.chargen = pcap_chargen::pcap_process_layer_chargen(layer, conn, *id);
        *id += 1;
      },
      "ssdp" => {
        let id = map_id.entry("ssdp").or_insert(0);
        pcap_packet.ssdp = pcap_ssdp::pcap_process_layer_ssdp(layer, conn, *id);
        *id += 1;
      },
      "eth" => {println!("layer is eth - we don't want nothing here")},
      "tcp" => {println!("layer is tcp - we don't want nothing here")},
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

  pcap_packet._insert(conn);
  return pcap_packet;
}
