
use rtshark::Layer;
use rusqlite::Connection;
use cidr_utils::cidr::Ipv4Cidr;
// use std::str::FromStr;
// use cidr_utils::cidr::IpCidr;

// https://www.wireshark.org/docs/dfref/d/dns.html

pub struct PcapIP {
  pub id: i32,
  pub vitima_addr: String,
  pub vitima_cidr: Ipv4Cidr,
}

pub fn drop_table(conn: &Connection) {
  let result = conn.execute(
    "DROP TABLE IF EXISTS PCAP_IP",
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
    "CREATE TABLE IF NOT EXISTS PCAP_IP (
      id INTEGER PRIMARY KEY,
      vitima_addr TEXT NOT NULL,
      vitima_cidr TEXT NOT NULL
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

impl PcapIP {
  pub fn default(id: i32) -> PcapIP {
    PcapIP {
      id,
      vitima_addr: String::from(""),
      vitima_cidr: Ipv4Cidr::from_str("192.168.51.0/24").unwrap(),
    }
  }

  pub fn insert(&self, conn: &Connection) {
    let result = conn.execute(
        "INSERT INTO PCAP_IP (vitima_addr, vitima_cidr) values (?1, ?2)",
        &[&self.vitima_addr.to_string(), &self.vitima_cidr.to_string()],
    );

    match result {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ip: {:?}", err)
      }
    }
  }
}

pub fn pcap_process_layer_ip(layer: Layer, conn: &Connection, id: i32) -> PcapIP {
  let mut pcap_ip = PcapIP::default(id);

  if layer.name() != "ip" {
    return pcap_ip;
  }

  println!("Processing ip");
  for metadata in layer {
    let (name, value, _display) = (metadata.name(), metadata.value(), metadata.display());
    match name {
      "ip.addr" => {
        pcap_ip.vitima_addr = value.to_string();
        let cidr = Ipv4Cidr::from_str(value);
        match cidr {
          Ok(ipcidr) => {pcap_ip.vitima_cidr = ipcidr},
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
      _ => {},
      // _ => println!("ignored field: {} = {} - {}", name, value, display),
    }
  }
  pcap_ip.insert(conn);
  return pcap_ip;
}
