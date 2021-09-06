use std::collections::HashMap;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use actix::{fut, ActorContext, ActorFuture};
use actix::{ContextFutureSpawner, WrapFuture};
use serde_json::Value;
use uuid::Uuid;
use serde::Deserialize;

use crate::app::{ClientMessage, client};
use crate::database::DbExecutor;
use crate::models;

/* -------------------------------------------------------------------------- */
/*                                    CHAT                                    */
/* -------------------------------------------------------------------------- */

// TODO: Add responsabilidades do chat;
pub struct Chat {
    pub id: Uuid,
    pub db_executor: Addr<DbExecutor>,
    message_counter: u64,
    members: HashMap<String, Addr<client::Client>>,
}

impl Chat {
    pub fn new(chat_model: models::chat::Chat, db_executor: Addr<DbExecutor>,) -> Self {
        Self {
            id: chat_model.id,
            db_executor,
            message_counter: 0,
            members: HashMap::new()
        }
    }

    pub fn  add_member(&mut self, fingerprint: String, addr: Addr<client::Client>) {
        self.members.insert(fingerprint, addr);
    }
}

impl Actor for Chat {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        for (_, addr) in self.members.iter() {
            addr.send(client::Joined(ctx.address()))
                .into_actor(self)
                .then(|res, _, ctx| {
                match res {
                    Ok(_res) =>(),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx)
        }
    }
}

/* ------------------ HANLDER TO PARSE MESSAGES FROM CLIENT ----------------- */

impl Handler<ClientMessage> for Chat {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        match msg.event.as_str() {
            "message" => {
                self.members.clone().iter().for_each(move |(_, addr)| {
                    addr.do_send(msg.clone());
                })
            },
            "exit" => (),
            _ => warn!("Invalid Message, {:#?}", msg)
        };
    }
}


/* ----------------------- HANDLE TO SEND USER TIMEOUT ---------------------- */
pub struct Timeout(pub String);

impl Message for Timeout {
    type Result = ();
} 

impl Handler<Timeout> for Chat {
    type Result = ();

    fn handle(&mut self, msg: Timeout, ctx: &mut Self::Context) -> Self::Result {
        info!("User time out")
    }
}

/* --------------------- HANDLE TO SEND USER DISCONNECT --------------------- */
pub struct Disconnect(pub String);

impl Message for Disconnect {
    type Result = ();
}

impl Handler<Disconnect> for Chat {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        info!("User disconecterd")
    }
}