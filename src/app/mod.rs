use actix::{Addr, Message};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::database;

pub mod client;
pub mod server;

#[derive(Clone)]
pub struct AppState {
    pub database: Addr<database::DbExecutor>,
    pub server: Addr<server::Server>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClientMessage {
    pub fingerprint: String,
    pub event: String,
    pub data: Value,
}

impl Message for ClientMessage {
    type Result = ();
}
