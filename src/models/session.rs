use std::net::Ipv4Addr;

use chrono::NaiveDateTime;

use crate::schema::sessions;

#[derive(Queryable, Identifiable)]
pub struct Session {
    pub id: String, // id as fingerprint
    pub address: ipnetwork::IpNetwork,
    pub last_connection: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub id: String, // id as fingerprint
    pub address: ipnetwork::IpNetwork,
}