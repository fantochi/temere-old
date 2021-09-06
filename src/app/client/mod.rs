use std::time::Duration;

use actix::clock::Instant;
use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix::{fut, ActorContext, ActorFuture};
use actix::{ContextFutureSpawner, WrapFuture};
use actix_web_actors::ws::{self, Message::Text};
use serde_json::Value;
use uuid::Uuid;

use super::server;
use super::ClientMessage;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_ALERT_TIMEOUT: Duration = Duration::from_secs(3);
const CLIENT_DISCONECT_TIMEOUT: Duration = Duration::from_secs(10);

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

            if Instant::now().duration_since(act.heart_beat) > CLIENT_DISCONECT_TIMEOUT {
                act.lobby_addr.do_send(server::lobby::Disconnect(act.fingerprint.clone()));
                ctx.stop();
                return;
            } 
            else if Instant::now().duration_since(act.heart_beat) > CLIENT_ALERT_TIMEOUT {
                
                println!("{:?}", Instant::now().duration_since(act.heart_beat));
                if let Some(chat_addr) = act.chat_addr.clone() {
                    chat_addr.do_send(server::chat::Timeout(act.fingerprint.clone()));
                }
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
        ctx.text(json!(msg).to_string()) 
    }
}

pub struct Joined(pub Addr<server::chat::Chat>);

impl Message for Joined {
    type Result = ();
}

impl Handler<Joined> for Client {
    type Result = ();

    fn handle(&mut self, msg: Joined, ctx: &mut Self::Context) -> Self::Result {
        self.chat_addr = Some(msg.0.clone());
        ctx.text(self.client_event_message("joined", None));
    }
}