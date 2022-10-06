// use rusqlite::{Connection, Result};
use postgres::{Client, NoTls};
use std::env; // https://crates.io/crates/postgres/0.17.0-alpha.1

// pub fn conn_get_mix_protocol() -> Result<Connection, rusqlite::Error> {
//   let current_path = match env::current_dir() {
//     Ok(curpath) => curpath,
//     Err(_) => PathBuf::from("./"),
//   };

//   let db_path = current_path
//     .as_path()
//     .join("../../db/database-2022-05-11/mix_protocol.sqlite");

//   println!("Opening connection to {}", db_path.display());
//   return Connection::open(db_path);
// }

// pub fn conn_get_rust_pcap() -> Result<Connection, rusqlite::Error> {
//   let current_path = match env::current_dir() {
//     Ok(curpath) => curpath,
//     Err(_) => PathBuf::from("./"),
//   };

//   let db_path = current_path
//     .as_path()
//     .join("../../db/database-2022-05-11/rust_pcap.sqlite");

//   println!("Opening connection to {}", db_path.display());
//   return Connection::open(db_path);
// }

pub fn conn_gcp_rust_pcap() -> Client {
  let db_pass = env::var("DB_PASS").unwrap();
  let uri = format!(
    "host=localhost user=postgres password={} port=5431",
    db_pass
  )
  .to_string();

  return Client::connect(&uri, NoTls).unwrap();
}

pub fn drop_tables(conn: &mut Client) {
  match conn.batch_execute(
    "
    DROP TABLE IF EXISTS PCAP_ATTACK;
    DROP TABLE IF EXISTS PCAP_ATTACK_PACKET;
    DROP TABLE IF EXISTS PCAP_PACKET;
    DROP TABLE IF EXISTS PCAP_IP;
    DROP TABLE IF EXISTS PCAP_UDP;
    DROP TABLE IF EXISTS PCAP_DNS;
    DROP TABLE IF EXISTS PCAP_LDAP;
    DROP TABLE IF EXISTS PCAP_NTP;
    ",
  ) {
    Ok(_) => {
      println!("drop Tables!")
    }
    Err(err) => {
      println!("Problem dropping table: {:?}", err)
    }
  }
}

pub fn journal_mode(conn: &mut Client) {
  match conn.batch_execute("PRAGMA journal_mode=WAL; PRAGMA synchronous=OFF;") {
    Ok(_) => {
      println!("journal_mode=WAL!")
    }
    Err(err) => {
      println!("Problem setting journal: {:?}", err)
    }
  }
}

pub fn create_tables(conn: &mut Client) {
  match conn.batch_execute(
    "
    CREATE TABLE IF NOT EXISTS PCAP_ATTACK (
      id BIGINT NOT NULL,
      ip_vitima_cidr TEXT NOT NULL,
      packets_per_attack INT NOT NULL,
      timestamp_inicio TEXT NOT NULL,
      timestamp_fim TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_PACKET (
      id BIGINT NOT NULL,
      timestamp_str TEXT NOT NULL,
      attack_id BIGINT NOT NULL,
      ip_id BIGINT NOT NULL,
      udp_id BIGINT NOT NULL,
      attack_type TEXT NOT NULL,
      dns_id BIGINT,
      ldap_id BIGINT,
      ntp_id BIGINT
    );

    CREATE TABLE IF NOT EXISTS PCAP_IP (
      id BIGINT NOT NULL,
      vitima_addr TEXT NOT NULL,
      vitima_cidr TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_UDP (
      id BIGINT NOT NULL,
      destination_port SMALLINT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_DNS (
      id BIGINT NOT NULL,
      tx_id INT NOT NULL,
      qname TEXT NOT NULL,
      qtype TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_LDAP (
      id BIGINT NOT NULL,
      message_id INT NOT NULL,
      protocol_op INT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_NTP (
      id BIGINT NOT NULL,
      refid BIGINT NOT NULL
    );
    ",
  ) {
    Ok(_) => {
      println!("create Tables!")
    }
    Err(err) => {
      println!("Problem creating table: {:?}", err)
    }
  }
}

// pub fn get_stmt_pcap_attack(conn: &Connection) -> Statement {
//   return conn.prepare("INSERT INTO PCAP_ATTACK (id, ip_vitima_cidr, packets_per_attack, timestamp_inicio, timestamp_fim) values (?, ?, ?, ?, ?)").unwrap();
// }

// pub fn get_stmt_pcap_attack_packet(conn: &mut Connection) -> Statement {
//   return conn
//     .prepare("INSERT INTO PCAP_ATTACK_PACKET (attack_id, packet_id) values (?, ?)")
//     .unwrap();
// }

// pub fn close(conn: Connection) {
//   match conn.close() {
//     Ok(_) => {
//       println!("Connection closed")
//     }
//     Err(_) => {
//       println!("Error closing connection")
//     }
//   }
// }
