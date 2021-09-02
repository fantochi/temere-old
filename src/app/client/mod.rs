use actix::{Actor, Addr, AsyncContext, StreamHandler};
use actix::{
    fut, ActorContext, ActorFuture, ContextFutureSpawner, 
 WrapFuture,
};
use actix_web_actors::ws::{self, Message::Text};

use super::server;
use super::ClientMessage;
// Use fingerprint to get websocket conn example: 6f53480fe064ff8f6f037df0c65e6fd7

pub struct Client {
    fingerprint: String,
    lobby_addr: Addr<server::lobby::Lobby>,
    chat_addr: Option<Addr<server::chat::Chat>>
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.lobby_addr
            .send(server::lobby::Connect {
                conn_addr: ctx.address(),
                fingerprint: self.fingerprint.clone(),
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx)
    }
}

impl Client {
    pub fn new(fingerprint: String, lobby_addr: Addr<server::lobby::Lobby>) -> Self {
        Self {
            fingerprint,
            lobby_addr,
            chat_addr: None
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => todo!(),
            Ok(ws::Message::Pong(_)) => todo!(),
            Ok(ws::Message::Binary(bin)) => todo!(),
            Ok(ws::Message::Close(reason)) => todo!(),
            Ok(ws::Message::Continuation(_)) => todo!(),
            Ok(ws::Message::Nop) => todo!(),
            Ok(Text(s)) => {
                let message_validator = serde_json::from_str::<ClientMessage>(s.as_str());

                match message_validator {
                    Err(e) => {
                        let res = ClientMessage{ 
                            event: String::from("error"),
                            data: json!({"message": "Bad Request", "code" : 400})
                        };
                        ctx.text(json!(res).to_string());
                    },

                    Ok(client_message) => {
                        match self.chat_addr.clone() {
                            Some(chat_add) => chat_add.do_send(client_message),
                            None => self.lobby_addr.do_send(client_message)
                        };
                    },
                }
            },
            Err(e) => panic!("{}", e),
        }
    }
}