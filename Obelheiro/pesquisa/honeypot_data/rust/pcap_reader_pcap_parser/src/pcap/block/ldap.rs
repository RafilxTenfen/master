use ldap_parser::ldap::LdapMessage;
// use tokio_postgres::Client;
use rusqlite::{params, Connection};

pub struct PcapLDAP {
  pub id: u32,
  pub message_id: u32,
  pub protocol_op: u32,
}

impl PcapLDAP {
  pub fn insert(&self, conn: &mut Connection) {
    match conn.execute(
      "INSERT INTO PCAP_LDAP (id, message_id, protocol_op) values ($1, $2, $3)",
      params![&self.id, &self.message_id, &self.protocol_op],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ldap: {:?}", err)
      }
    }
  }
}

pub fn process_ldap(ldap_message: &LdapMessage, id: u32) -> PcapLDAP {
  return PcapLDAP {
    id,
    message_id: ldap_message.message_id.0,
    protocol_op: ldap_message.protocol_op.tag().0,
  };
}
