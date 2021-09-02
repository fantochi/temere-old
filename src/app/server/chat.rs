use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};
use actix::{fut, ActorContext, ActorFuture};
use actix::{ContextFutureSpawner, WrapFuture};
use uuid::Uuid;

use crate::app::{ClientMessage, client::Client};

pub struct Chat {
    pub id: Uuid,
    members: HashMap<String, Addr<Client>>,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            members: HashMap::new()
        }
    }

    pub fn  add_member(&mut self, fingerprint: String, addr: Addr<Client>) {
        self.members.insert(fingerprint, addr);
    }
}

impl Actor for Chat {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        for (fingerprint, addr) in self.members.iter() {
            addr.send(ClientMessage { 
                    fingerprint: fingerprint.clone(),
                    event: String::from("joined"), 
                    data: json!({})
                })
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

impl Handler<ClientMessage> for Chat {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        match msg.event.as_str() {
            "message" => (),
            "exit" => todo!(),
            _ => warn!("Invalid Message, {:#?}", msg)
        };
    }
}

