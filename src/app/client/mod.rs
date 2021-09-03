use std::time::Duration;

use actix::clock::Instant;
use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix::{fut, ActorContext, ActorFuture};
use actix::{ContextFutureSpawner, WrapFuture};
use actix_web_actors::ws::{self, Message::Text};
use serde_json::Value;

use super::server;
use super::ClientMessage;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// Use fingerprint to get websocket conn example: 6f53480fe064ff8f6f037df0c65e6fd7
// TODO: Create method to parse and verify if finger print is valid
pub struct Client {
    fingerprint: String,
    lobby_addr: Addr<server::lobby::Lobby>,
    chat_addr: Option<Addr<server::chat::Chat>>,
    heart_beat: Instant,
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        self.heart_beat(ctx);

        // Send message to Lobby for try Connect
        self.lobby_addr
            .send(server::lobby::Connect {
                conn_addr: ctx.address(),
                fingerprint: self.fingerprint.clone(),
            })
            .into_actor(self)
            .then(|res, client, ctx| {
                match res {
                    Ok(_res) => match _res {
                        Err(_) => {
                            ctx.text(client.client_error_message(400, "Client aready connected."));
                            ctx.stop();
                        },
                        Ok(_) => ctx.text(client.client_event_message("connected", None))
                    },
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx)
    }
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        self.lobby_addr.do_send(server::lobby::Disconnect(self.fingerprint.clone()));

        actix::Running::Stop
    }
}

impl Client {
    pub fn new(fingerprint: String, lobby_addr: Addr<server::lobby::Lobby>) -> Self {
        Self {
            fingerprint,
            lobby_addr,
            chat_addr: None,
            heart_beat: Instant::now(),
        }
    }

    fn client_error_message(&self, code: u64, message: &'static str) -> String { 
        json!(ClientMessage{
            fingerprint: self.fingerprint.clone(),
            event: String::from("error"),
            data: json!({"code": code, "message" : message})
        }).to_string()
    }

    fn client_event_message(&self, event: &'static str, data: Option<Value>) -> String {

        let data = match data {
            Some(value) => value,
            None => json!({})
        };

        json!(ClientMessage{
            fingerprint: self.fingerprint.clone(),
            event: String::from(event),
            data: data
        }).to_string()
    }

    fn heart_beat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heart_beat) > CLIENT_TIMEOUT {
                // if let Some(chat_addr) = act.chat_addr.clone() {
                //     // TODO: Make Timeout Message
                //     act.lobby_addr.do_send(Disconnect { id: act.id });
                // };
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heart_beat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heart_beat = Instant::now();
            }
            Ok(ws::Message::Binary(_)) => (),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => {
                let message_validator = serde_json::from_str::<ClientMessage>(s.as_str());

                match message_validator {
                    Err(e) => {
                        error!("Message validator error: {}", e);
                        ctx.text(self.client_error_message(400, "Bad Request"));
                    },                  
                    Ok(client_message) => {

                        if client_message.fingerprint != self.fingerprint.clone() {
                            ctx.text(self.client_error_message(401, "-12315"));
                            return;
                        }

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

impl Handler<ClientMessage> for Client {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.event.clone().as_str() {
            "lobby-closed" => {
                match self.chat_addr {
                    Some(_) => (),
                    None => {
                        ctx.text(json!(msg).to_string());
                        ctx.stop();
                    }
                }
            },
            _ => ctx.text(json!(msg).to_string())
        }
    }
}