use rusqlite::{Connection, Result};
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

}

pub fn create_tables(conn: &Connection) {

}

pub fn close(conn: Connection) {
  match conn.close() {
    Ok(_) => {
      println!("Connection closed")
    },
    Err(_) => todo!(),
  }
}