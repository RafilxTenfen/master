// use postgres::tls::NoTlsStream;
// use postgres::{Socket, tls::NoTlsStream};
use std::path::PathBuf;
use rusqlite::{Connection, Result};
// use tokio_postgres::{Client, NoTls};
// use tokio_postgres::{Client, NoTls, Socket};

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

pub fn conn_get_rust_pcap() -> Result<rusqlite::Connection, rusqlite::Error> {
  let current_path = match env::current_dir() {
    Ok(curpath) => curpath,
    Err(_) => PathBuf::from("./"),
  };

  let db_path = current_path
    .as_path()
    .join("../../db/database-2022-05-11/rust_pcap.sqlite");

  println!("Opening connection to {}", db_path.display());
  return rusqlite::Connection::open(db_path);
}

// pub async fn conn_gcp_async_rust_pcap() -> (tokio_postgres::Client, tokio_postgres::Connection<Socket, NoTlsStream>) {
//   let db_pass = env::var("DB_PASS").unwrap();
//   let uri = format!(
//     "host=localhost user=postgres password={} port=5431",
//     db_pass
//   )
//   .to_string();

//   return tokio_postgres::connect(&uri, NoTls).await.unwrap();
// }

// pub fn conn_gcp_rust_pcap() -> Client {
//   let db_pass = env::var("DB_PASS").unwrap();
//   let uri = format!(
//     "host=localhost user=postgres password={} port=5431",
//     db_pass
//   )
//   .to_string();

//   return Client::connect(&uri, NoTls).unwrap();
// }

pub fn drop_tables(conn: &mut Connection) {
  match conn.execute_batch(
    "
    DROP TABLE IF EXISTS PCAP_ATTACK;
    DROP TABLE IF EXISTS PCAP_ATTACK_PACKET;
    DROP TABLE IF EXISTS PCAP_PACKET;
    DROP TABLE IF EXISTS PCAP_IP;
    DROP TABLE IF EXISTS PCAP_UDP;
    DROP TABLE IF EXISTS PCAP_DNS;
    DROP TABLE IF EXISTS PCAP_LDAP;
    DROP TABLE IF EXISTS PCAP_NTP;
    DROP TABLE IF EXISTS TBIP;
    DROP TABLE IF EXISTS TBCIDR;
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

pub fn journal_mode(conn: &mut Connection) {
  match conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=OFF;") {
    Ok(_) => {
      println!("journal_mode=WAL!")
    }
    Err(err) => {
      println!("Problem setting journal: {:?}", err)
    }
  }
}

pub fn create_tables(conn: &mut Connection) {
  match conn.execute_batch(
    "
    CREATE TABLE IF NOT EXISTS PCAP_ATTACK (
      id INT NOT NULL,
      vitima_cidr_id INT NOT NULL,
      packets_per_attack INT NOT NULL,
      timestamp_inicio TEXT NOT NULL,
      timestamp_fim TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_PACKET (
      id INT NOT NULL,
      timestamp_str TEXT NOT NULL,
      attack_id INT NOT NULL,
      ip_id INT NOT NULL,
      udp_id INT NOT NULL,
      attack_type TEXT NOT NULL,
      dns_id INT,
      ldap_id INT,
      ntp_id INT
    );

    CREATE TABLE IF NOT EXISTS PCAP_IP (
      id INT NOT NULL,
      vitima_addr_id INT NOT NULL,
      vitima_cidr_id INT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_UDP (
      id INT NOT NULL,
      destination_port SMALLINT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_DNS (
      id INT NOT NULL,
      tx_id INT NOT NULL,
      qname TEXT NOT NULL,
      qtype TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_LDAP (
      id INT NOT NULL,
      message_id INT NOT NULL,
      protocol_op INT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_NTP (
      id INT NOT NULL,
      refid INT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS TBIP (
      id INT NOT NULL,
      ip TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS TBCIDR (
      id INT NOT NULL,
      cidr TEXT NOT NULL
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

// pub async fn disable_vacuum(conn: &mut Connection) {
//   match conn.execute_batch(
//     "
//     ALTER TABLE PCAP_ATTACK SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );

//     ALTER TABLE PCAP_PACKET SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );

//     ALTER TABLE PCAP_IP SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );

//     ALTER TABLE PCAP_UDP SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );

//     ALTER TABLE PCAP_DNS SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );

//     ALTER TABLE PCAP_LDAP SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );

//     ALTER TABLE PCAP_NTP SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );

//     ALTER TABLE TBIP SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );

//     ALTER TABLE TBCIDR SET (
//       autovacuum_enabled = false,
//       toast.autovacuum_enabled = false
//     );
//     ",
//   ) {
//     Ok(_) => {
//       println!("disable autovacuum Tables!")
//     }
//     Err(err) => {
//       println!("Problem setting autovacuum table: {:?}", err)
//     }
//   }
// }

// pub fn get_stmt_pcap_attack(conn: &Connection) -> Statement {
//   return conn.prepare("INSERT INTO PCAP_ATTACK (id, ip_vitima_cidr, packets_per_attack, timestamp_inicio, timestamp_fim) values (?, ?, ?, ?, ?)").unwrap();
// }

// pub fn get_stmt_pcap_attack_packet(conn: &mut Connection) -> Statement {
//   return conn
//     .prepare("INSERT INTO PCAP_ATTACK_PACKET (attack_id, packet_id) values (?, ?)")
//     .unwrap();
// }

pub fn close(conn: rusqlite::Connection) {
  match conn.close() {
    Ok(_) => {
      println!("Connection closed")
    }
    Err(_) => {
      println!("Error closing connection")
    }
  }
}
