use std::collections::HashMap;

use actix::{Actor, Context, Handler};
use uuid::Uuid;

use crate::app::ClientMessage;

pub struct Chat {
    id: Uuid,
    members: HashMap<String, String>
}

impl Actor for Chat {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for Chat {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.event.as_str() {
            "message" => todo!(),
            "exit" => todo!(),
            _ => ()            
        };
    }
}