use ldap_parser::ldap::LdapMessage;

pub struct PcapLDAP {
  pub id: u32,
  pub message_id: u32,
  pub protocol_op: u32,
}

pub fn process_ldap(ldap_message: &LdapMessage, id: u32) -> PcapLDAP {
  return PcapLDAP {
    id,
    message_id: ldap_message.message_id.0,
    protocol_op: ldap_message.protocol_op.tag().0,
  };
}
