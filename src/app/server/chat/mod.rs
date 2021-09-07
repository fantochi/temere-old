use std::collections::HashMap;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use actix::{fut, ActorContext, ActorFuture};
use actix::{ContextFutureSpawner, WrapFuture};
use serde_json::Value;
use uuid::Uuid;
use serde::Deserialize;

use crate::app::{ClientMessage, client};
use crate::database::{self, DbExecutor};
use crate::models;

/* -------------------------------------------------------------------------- */
/*                                    CHAT                                    */
/* -------------------------------------------------------------------------- */

// TODO: Add responsabilidades do chat;
pub struct Chat {
    pub id: Uuid,
    pub db_executor: Addr<DbExecutor>,
    message_counter: i32,
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

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("Chat finalizado.");
        self.db_executor.do_send(database::chat::CloseChat{
            chat_id: self.id,
            message_counter: self.message_counter
        });
    }
}

/* ------------------ HANLDER TO PARSE MESSAGES FROM CLIENT ----------------- */

impl Handler<ClientMessage> for Chat {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.event.as_str() {
            "message" => {
                if self.members.len() >= 2 {
                    self.message_counter += 1;
                    self.members.clone().iter().for_each(move |(fg, addr)| {
                        let mut a = msg.clone();
                        a.fingerprint = fg.clone();
                        addr.do_send(a.clone());
                    })
                }
            },
            "exit" => {
                match self.members.remove_entry(&msg.fingerprint) {
                    Some((fg, addr)) => {
        
                        info!("{} saiu do chat", &msg.fingerprint);

                        addr.do_send(ClientMessage {
                            event: String::from("exited"),
                            data: json!({}),
                            fingerprint: fg
                        });

                        if self.members.len() <= 0 {
                            ctx.stop();
                            return;   
                        }
        
                        self.members.clone().iter().for_each(move |(fingerprint, addr)| {
                                addr.do_send(ClientMessage {
                                    fingerprint: fingerprint.clone(),
                                    event: "disconnected".to_string(),
                                    data: json!({})
                                })
                        })
                        
                    },
                    None => ()
                }
            },
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
        self.members.clone().iter().for_each(move |(fingerprint, addr)| {
            if fingerprint.clone() != msg.0.clone() {
                info!("{} | {}", fingerprint.clone(), msg.0.clone());
                addr.do_send(ClientMessage {
                    fingerprint: fingerprint.clone(),
                    event: "timeout".to_string(),
                    data: json!({})
                })                
            }
        })
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
        match self.members.remove_entry(&msg.0) {
            Some((disconnected_fingerprint,_ )) => {

                info!("{} foi desconectado do chat", disconnected_fingerprint);

                if self.members.len() <= 0 {
                    ctx.stop();
                    return;   
                }

                self.members.clone().iter().for_each(move |(fingerprint, addr)| {
                        addr.do_send(ClientMessage {
                            fingerprint: fingerprint.clone(),
                            event: "disconnected".to_string(),
                            data: json!({})
                        })
                })
            },
            None => ()
        }
    }
}