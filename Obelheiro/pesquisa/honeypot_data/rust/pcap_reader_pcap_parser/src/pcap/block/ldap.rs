use ldap_parser::ldap::LdapMessage;
use tokio_postgres::Client;
// use rusqlite::{params, Connection};

pub struct PcapLDAP {
  pub id: i32,
  pub message_id: i32,
  pub protocol_op: i32,
}

impl PcapLDAP {
  pub fn insert(&self, conn: &mut Client) {
    match conn.execute(
      "INSERT INTO PCAP_LDAP (id, message_id, protocol_op) values ($1, $2, $3)",
      &[&self.id, &self.message_id, &self.protocol_op],
    ) {
      Ok(_) => {}
      Err(err) => {
        println!("Problem inserting ldap: {:?}", err)
      }
    }
  }
}

pub fn process_ldap(ldap_message: &LdapMessage, id: i32) -> PcapLDAP {
  return PcapLDAP {
    id,
    message_id: ldap_message.message_id.0 as i32,
    protocol_op: ldap_message.protocol_op.tag().0 as i32,
  };
}
