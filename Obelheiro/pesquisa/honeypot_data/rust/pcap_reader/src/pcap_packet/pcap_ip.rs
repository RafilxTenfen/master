use cidr_utils::cidr::Ipv4Cidr;
use rtshark::{Layer, Metadata};
use rusqlite::{params, Connection};
// use std::str::FromStr;
// use cidr_utils::cidr::IpCidr;

// https://www.wireshark.org/docs/dfref/d/dns.html

pub struct PcapIP {
  pub id: i32,
  pub vitima_addr: String,
  pub vitima_cidr: Ipv4Cidr,
}

pub fn drop_table(conn: &Connection) {
  match conn.execute("DROP TABLE IF EXISTS PCAP_IP", []) {
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
    "CREATE TABLE IF NOT EXISTS PCAP_IP (
      id INTEGER NOT NULL,
      vitima_addr TEXT NOT NULL,
      vitima_cidr TEXT NOT NULL
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

impl PcapIP {
  pub fn default(id: i32) -> PcapIP {
    PcapIP {
      id,
      vitima_addr: String::from(""),
      vitima_cidr: Ipv4Cidr::from_str("192.168.51.0/24").unwrap(),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    match conn.execute(
      "INSERT INTO PCAP_IP (id, vitima_addr, vitima_cidr) values (?1, ?2, ?3)",
      params![&self.id, &self.vitima_addr, &self.vitima_cidr.to_string()],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ip: {:?}", err)
      }
    }
  }

  pub fn metadata_process(&mut self, metadata: &Metadata) {
    let (name, value) = (metadata.name(), metadata.value());
    match name {
      "ip.addr" => {
        self.vitima_addr = value.to_string();
        match Ipv4Cidr::from_str(value) {
          Ok(ipcidr) => self.vitima_cidr = ipcidr,
          Err(err) => {
            println!("Problem Ipv4Cidr ip: {:?}, - err {:?}", value, err)
          }
        }
        // println!("ip: {} cidr = {}", pcap_ip.vitima_addr, pcap_ip.vitima_cidr)
        // Ipv4Cidr::new_host(value.parse()::<Ipv4Addr>)
        // let addr = Address::Cidr(value);
        // Ipv4Cidr::new(Address::Inet(), len)
        // let ipv4_addr = "012.004.002.000".parse::<Ipv4Addr>().unwrap();
        //  AnyIpCidr::new_host(value)
      }
      _ => {
        println!("{}", name)
      } // _ => {} // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
}

pub fn pcap_process_layer_ip(layer: &Layer, id: &i32) -> PcapIP {
  let mut pcap_ip = PcapIP::default(*id);

  // println!("Processing ip");
  layer
    .iter()
    .for_each(|metadata| pcap_ip.metadata_process(metadata));

  return pcap_ip;
}
