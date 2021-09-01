pub mod chat;
pub mod lobby;

use std::collections::HashMap;
use actix::{Actor, Addr, Context, Handler, Message};
use uuid::Uuid;

use crate::database::DbExecutor;

pub struct Server {
    lobbys: HashMap<Uuid, Addr<lobby::Lobby>>
}

impl Server {
    pub fn new() -> Self {
        Self {
            lobbys: HashMap::new()
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

//========================
//       ACTIONS
//========================
