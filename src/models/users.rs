use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
    pub id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: String
}