mod chat;

use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};
use uuid::Uuid;

pub struct Server {
    chats: HashMap<Uuid, Addr<chat::Chat>>
}

impl Server {
    pub fn new() -> Self {
        Self {
            chats: HashMap::new()
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

