pub mod actions;

use actix::{Actor, Addr, StreamHandler};
use actix_web_actors::ws;


use super::server;
// Use fingerprint to get websocket conn example: 6f53480fe064ff8f6f037df0c65e6fd7

pub struct Client {
    fingerprint: String,
    server_addr: Addr<server::Server>,
    chat_addr: Option<String>
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;
}

impl Client {
    pub fn new(fingerprint: String, server_addr: Addr<server::Server>) -> Self {
        Self {
            fingerprint,
            server_addr,
            chat_addr: None
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => todo!(),
            Ok(ws::Message::Pong(_)) => todo!(),
            Ok(ws::Message::Binary(bin)) => todo!(),
            Ok(ws::Message::Close(reason)) => todo!(),
            Ok(ws::Message::Continuation(_)) => todo!(),
            Ok(ws::Message::Nop) => todo!(),
            Ok(Text(s)) => todo!(),
            Err(e) => panic!("{}", e),
        }
    }
}