use uuid::Uuid;
use serde::Serialize;

use crate::schema::lobbys;

#[derive(Debug, Queryable, Identifiable, Serialize)]
pub struct Lobby {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub nsfw: bool,
    pub enabled: bool
}