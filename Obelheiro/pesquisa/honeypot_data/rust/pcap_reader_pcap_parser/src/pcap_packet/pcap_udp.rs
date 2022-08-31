use rtshark::{Layer, Metadata};
use rusqlite::{params, Connection};

// https://www.wireshark.org/docs/dfref/d/dns.html

pub struct PcapUDP {
  pub id: i32,
  pub destination_port: i32,
}

pub fn drop_table(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_UDP", []) {
    Ok(_) => {
      println!("Table created!")
    }
    Err(err) => {
      println!("Problem droping table: {:?}", err)
    }
  }
}

pub fn create_table(conn: &Connection) {
  match conn.execute(
    "CREATE TABLE IF NOT EXISTS PCAP_UDP (
      id INTEGER NOT NULL,
      destination_port INTEGER NOT NULL
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

impl PcapUDP {
  pub fn default(id: i32) -> PcapUDP {
    PcapUDP {
      id,
      destination_port: 0,
    }
  }

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_UDP (id, destination_port) values (?1, ?2)",
      params![&self.id, &self.destination_port],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting udp: {:?}", err)
      }
    }
  }

  pub fn metadata_process(&mut self, metadata: &Metadata) {
    let (name, value) = (metadata.name(), metadata.value());
    match name {
      "udp.dstport" => match value.parse::<i32>() {
        Ok(destination_port) => {
          self.destination_port = destination_port;
        }
        Err(err) => {
          println!("Problem parse dns.id: {:?}", err)
        }
      },
      _ => {} // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
}

pub fn pcap_process_layer_udp(layer: &Layer, id: &i32) -> PcapUDP {
  let mut pcap_udp = PcapUDP::default(*id);

  // if layer.name() != "udp" {
  //   return pcap_udp;
  // }

  // println!("Processing udp");
  layer
    .iter()
    .for_each(|metadata| pcap_udp.metadata_process(metadata));

  return pcap_udp;
}
