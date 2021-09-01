use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::chats;

#[derive(Debug, Queryable, Identifiable)]
pub struct Chat {
    pub id: Uuid,
    pub message_counter: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
#[table_name = "chats"]
pub struct NewChat {
    pub id: Uuid,
    pub message_counter: i32,
    pub status: String
}