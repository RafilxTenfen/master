use rtshark::Layer;
use rusqlite::Connection;

// https://www.wireshark.org/docs/dfref/d/dns.html

pub struct PcapUDP {
  pub id: i32,
  pub destination_port: i32,
}

pub fn drop_table(conn: &Connection) {
  let result = conn.execute(
    "DROP TABLE IF EXISTS PCAP_UDP",
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
    "CREATE TABLE IF NOT EXISTS PCAP_UDP (
      id INTEGER PRIMARY KEY,
      destination_port INTEGER NOT NULL
     )",
    [],
  );

  match result {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem creating table: {:?}", err)
    }
  }
}

impl PcapUDP {
  pub fn default(id: i32) -> PcapUDP {
    PcapUDP {
      id,
      destination_port: 0,
    }
  }

  pub fn insert(&self, conn: &Connection) {
    if self.destination_port == 0 {
      return
    }

    let result = conn.execute(
        "INSERT INTO PCAP_UDP (id, destination_port) values (?1, ?2)",
        &[&self.id.to_string(), &self.destination_port.to_string()],
    );

    match result {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting udp: {:?}", err)
      }
    }
  }
}

pub fn pcap_process_layer_udp(layer: Layer, conn: &Connection, id: i32) -> PcapUDP {
  let mut pcap_udp = PcapUDP::default(id);

  if layer.name() != "udp" {
    return pcap_udp;
  }

  println!("Processing udp");
  for metadata in layer {
    let (name, value, _display) = (metadata.name(), metadata.value(), metadata.display());
    match name {
      "udp.dstport" => {
        pcap_udp.destination_port = value.parse::<i32>().unwrap();
      }
      _ => {},
      // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
  pcap_udp.insert(conn);
  return pcap_udp;
}
