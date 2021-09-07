mod session;

use std::collections::{HashMap, HashSet};
use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix::{fut, ActorContext, ActorFuture, Context};
use actix::{ContextFutureSpawner, WrapFuture};
use uuid::Uuid;

use crate::app::server::chat;
use crate::{app::{ClientMessage, client::Client}, database::{self, DbExecutor}, models};

use self::session::Session;
use super::chat::Chat;

pub struct Lobby {
    id: Uuid,
    db_executor: Addr<DbExecutor>,
    chats: HashMap<Uuid, Addr<Chat>>,
    sessions: HashMap<String, session::Session>
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

/* -------------------------------------------------------------------------- */
/*                                    LOBBY                                   */
/* -------------------------------------------------------------------------- */

// TODO: Adicionar descriCão das responsabilidades do Lobby

impl Lobby {
    pub fn new(lobby_model: models::lobby::Lobby, db_executor: Addr<DbExecutor>) -> Self {
        Self {
            id: lobby_model.id.clone(),
            db_executor,
            chats: HashMap::new(),
            sessions: HashMap::new()
        }
    }
}


/* --------------------- HANDLER TO PARSE CLIENT MESSAGE -------------------- */
impl Handler<ClientMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        // Message Events
        match msg.clone().event.as_str() {
            "look" => {
                let blacklist: HashSet<String> = HashSet::new();
                // Try find other user to make chat
                for (other_user_fingerprint, other_user_session) in self.sessions.clone() {

                    if other_user_fingerprint == msg.fingerprint.clone() {
                        continue;
                    }

                    if let Some(_) = blacklist.get(&other_user_fingerprint) {
                        continue;
                    }                    

                    match other_user_session.get_state() {
                        session::SessionState::Looking => {

                            self.db_executor
                                .send(database::chat::RegisterChat(self.id.clone()))
                                .into_actor(self)
                                .then( move |res, server, ctx| {
                                    match res {
                                        Ok(res) => {
                                            let chat_model = res.unwrap();
                                            let mut new_chat = Chat::new(chat_model, server.db_executor.clone());
                                            let lock_session = server.sessions.get_mut(&other_user_fingerprint);

                                            match lock_session {
                                                Some(mut_other_user_session) => {
                                                    mut_other_user_session.set_state(session::SessionState::InChat(new_chat.id));
                                                    new_chat.add_member(other_user_fingerprint.clone(), other_user_session.get_addr());

                                                    let my_self_session = server.sessions.get_mut(&msg.fingerprint).unwrap(); 
                                                    my_self_session.set_state(session::SessionState::InChat(new_chat.id));
                                                    new_chat.add_member(msg.fingerprint.clone(), my_self_session.get_addr());

                                                    server.chats.insert(new_chat.id.clone(), new_chat.start());
                                                },
                                                None => ()
                                            };
                                        },
                                        Err(e) => ()
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx);
                            return;
                        },
                        _ => ()
                    }
                }

                self.sessions.clone().iter().for_each(|(a, c)| {

                    println!("{:#?}", a);
                });

                let my_self_session = self.sessions.get_mut(&msg.fingerprint);

                if let Some(mut_session) = my_self_session {
                    match mut_session.get_state() {
                        session::SessionState::Waiting => {
                            mut_session.set_state(session::SessionState::Looking);
                            mut_session.get_addr().do_send(ClientMessage { fingerprint: msg.fingerprint.clone(), event: "looking".to_string(), data: serde_json::Value::default() });
                        },
                        _ => ()
                    }
                }                
            },
            _ => warn!(target: "Lobby", "ClientMessage Handler -> Invalid Command: \n{:#?}", msg)           
        };
    }
}



/* ----------------- HANDLER TO STORE USER SESSION ON LOBBY ----------------- */
pub struct Connect {
    pub fingerprint: String,
    pub conn_addr: Addr<Client>
}

impl Message for Connect {
    type Result = Result<(), ()>;
}

impl Handler<Connect> for Lobby {
    type Result = Result<(), ()>;
    
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        match self.sessions.insert(msg.fingerprint, Session::new(msg.conn_addr)) {
            Some(_) => Err(()),
            None => Ok(())
        }
    }
}


/* ---------------- HANDLER TO DISCONECT USER SESSION FROM LOBBY --------------- */
pub struct Disconnect(pub String);

impl Message for Disconnect {
    type Result = ();
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        //TODO: Add message to chat if user session == SessionState::inChat(chat_id)

        info!("{} foi desconectado do lobby", msg.0.clone());
        match self.sessions.remove_entry(&msg.0) {
            Some((fingerprint, session)) => {
                match session.get_state() {
                    session::SessionState::InChat(chat_id) => {
                        if let Some(chat_addr) = self.chats.get(&chat_id) {
                            chat_addr.do_send(chat::Disconnect(fingerprint));            
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

/* --------------------- HANDLER TO CHANGE SESSION STATE -------------------- */
pub struct ChatDisconnect(pub String);

impl Message for ChatDisconnect {
    type Result = ();
}

impl Handler<ChatDisconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ChatDisconnect, ctx: &mut Self::Context) -> Self::Result {
        match self.sessions.get_mut(&msg.0) {
            Some(session) => session.set_state(session::SessionState::Waiting),
            _ => ()
        }
    }
}