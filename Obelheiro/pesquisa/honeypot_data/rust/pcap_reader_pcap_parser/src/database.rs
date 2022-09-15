use rusqlite::{Connection, Result, Statement};
use std::{env, path::PathBuf};

pub fn conn_get_mix_protocol() -> Result<Connection, rusqlite::Error> {
  let current_path = match env::current_dir() {
    Ok(curpath) => curpath,
    Err(_) => PathBuf::from("./"),
  };

  let db_path = current_path
    .as_path()
    .join("../../db/database-2022-05-11/mix_protocol.sqlite");

  println!("Opening connection to {}", db_path.display());
  return Connection::open(db_path);
}

pub fn drop_tables(conn: &Connection) {
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

pub fn journal_mode(conn: &Connection) {
  match conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=OFF;") {
    Ok(_) => {
      println!("journal_mode=WAL!")
    }
    Err(err) => {
      println!("Problem setting journal: {:?}", err)
    }
  }
}

pub fn create_tables(conn: &Connection) {
  match conn.execute_batch(
    "
    CREATE TABLE IF NOT EXISTS PCAP_ATTACK (
      id INTEGER NOT NULL,
      ip_vitima_cidr TEXT NOT NULL,
      packets_per_attack INTEGER NOT NULL,
      timestamp_inicio TEXT NOT NULL,
      timestamp_fim TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_ATTACK_PACKET (
      attack_id INTEGER NOT NULL,
      packet_id INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_PACKET (
      id INTEGER NOT NULL,
      timestamp_str TEXT NOT NULL,
      ip_id INTEGER NOT NULL,
      udp_id INTEGER NOT NULL,
      attack_type TEXT NOT NULL,
      dns_id INTEGER,
      ldap_id INTEGER,
      ntp_id INTEGER
    );

    CREATE TABLE IF NOT EXISTS PCAP_IP (
      id INTEGER NOT NULL,
      vitima_addr TEXT NOT NULL,
      vitima_cidr TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_UDP (
      id INTEGER NOT NULL,
      destination_port INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_DNS (
      id INTEGER NOT NULL,
      tx_id INTEGER NOT NULL,
      qname TEXT NOT NULL,
      qtype TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_LDAP (
      id INTEGER NOT NULL,
      message_id INTEGER NOT NULL,
      protocol_op INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS PCAP_NTP (
      id INTEGER NOT NULL,
      refid INTEGER NOT NULL
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

pub fn close(conn: Connection) {
  match conn.close() {
    Ok(_) => {
      println!("Connection closed")
    }
    Err(_) => {
      println!("Error closing connection")
    }
  }
}
