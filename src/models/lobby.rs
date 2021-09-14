use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::lobbys;

#[derive(Queryable, PartialEq, Debug, Identifiable, Serialize)]
pub struct Lobby {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub nsfw: bool,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "lobbys"]
pub struct NewLobby {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub nsfw: bool,
    pub enabled: bool,
}

#[derive(Debug, AsChangeset)]
#[table_name = "lobbys"]
pub struct UpdateLobby {
    pub name: Option<String>,
    pub description: Option<String>,
    pub nsfw: Option<bool>,
    pub enabled: Option<bool>,
}
