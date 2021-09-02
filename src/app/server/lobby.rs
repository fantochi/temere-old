use std::collections::HashMap;
use actix::{Actor, Addr, Context, Handler, Message};
use uuid::Uuid;

use crate::app::{ClientMessage, client::Client};

pub struct Lobby {
    chats: HashMap<String, String>,
    sessions: HashMap<String, (Addr<Client>, Option<Uuid>)>
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            chats: HashMap::new(),
            sessions: HashMap::new()
        }
    }
}

// message parser

impl Handler<ClientMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.event.as_str() {
            "join" => todo!(),
            _ => ()            
        };
    }
}

// Connect user

pub struct Connect {
    pub fingerprint: String,
    pub conn_addr: Addr<Client>
}

impl Message for Connect {
    type Result = Result<(), ()>;
}

impl Handler<Connect> for Lobby {
    type Result = Result<(), ()>;
    
    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        match self.sessions.insert(msg.fingerprint, (msg.conn_addr.clone(), None)) {
            Some(_) => Err(()),
            None => Ok(())
        }
    }
}
