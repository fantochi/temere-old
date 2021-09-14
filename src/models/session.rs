use crate::schema::sessions;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable)]
pub struct Session {
    pub id: String, // id as fingerprint
    pub address: ipnetwork::IpNetwork,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub id: String, // id as fingerprint
    pub address: ipnetwork::IpNetwork,
}
