mod session;

use std::collections::{HashMap, HashSet};
use actix::{Actor, Addr, Context, Handler, Message};
use uuid::Uuid;

use crate::app::{ClientMessage, client::Client};

use self::session::{Session, SessionState};

use super::chat::Chat;

pub struct Lobby {
    enabled: bool,
    chats: HashMap<Uuid, Addr<Chat>>,
    sessions: HashMap<String, session::Session>
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Lobby {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            chats: HashMap::new(),
            sessions: HashMap::new()
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   ACTIONS                                  */
/* -------------------------------------------------------------------------- */

// Client message handler
impl Handler<ClientMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {

        if self.enabled.clone() == false {
            for (fingerprint, session) in self.sessions.iter() {
                session.get_addr().do_send(ClientMessage { fingerprint: fingerprint.clone(), event: String::from("lobby-closed"), data: json!({}) });
            }
            return;
        }

        // Message Events
        match msg.event.as_str() {
            "look" => {
                // TODO: Ignore if myself aready looking
                // TODO: Add 5 recent clients and blocked clients
                let blacklist: HashSet<String> = HashSet::new();

                for (fingerprint, session) in self.sessions.clone() {

                    if let Some(_) = blacklist.get(&fingerprint) {
                        continue;
                    }

                    if fingerprint == msg.fingerprint {
                        continue;
                    }

                    match session.get_state() {
                        session::SessionState::Looking => {
                            let mut new_chat = Chat::new();
                            
                            match self.sessions.get_mut(&fingerprint) {
                                Some(other_user_session) => {
                                    match other_user_session.get_state() { 
                                        
                                        SessionState::Looking => {
                                            other_user_session.set_state(SessionState::InChat(new_chat.id));
                                            new_chat.add_member(fingerprint.clone(), other_user_session.get_addr());

                                            let my_self_session = self.sessions.get_mut(&msg.fingerprint).unwrap(); 
                                            my_self_session.set_state(SessionState::InChat(new_chat.id));
                                            new_chat.add_member(msg.fingerprint.clone(), my_self_session.get_addr());
                                            

                                            self.chats.insert(new_chat.id.clone(), new_chat.start());
                                            return;
                                        },
                                        _=> continue
                                    }
                                },
                                None => continue
                            };
                        },
                        _ => ()
                    }
                }

                let my_self_session = self.sessions.get_mut(&msg.fingerprint).unwrap(); 
                my_self_session.set_state(SessionState::Looking);
            },
            _ => warn!("Invalid Message, {:#?}", msg)           
        };
    }
}

// Connect Client for Register Session
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

        if self.enabled == false {
            return Err(());
        }

        match self.sessions.insert(msg.fingerprint, Session::new(msg.conn_addr)) {
            Some(_) => Err(()),
            None => Ok(())
        }
    }
}


pub struct Enabled(pub bool);

impl Message for Enabled {
    type Result = ();
}

impl Handler<Enabled> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Enabled, _ctx: &mut Self::Context) -> Self::Result {
        self.enabled = msg.0;

        if msg.0 == false {
            for (fingerprint, session) in self.sessions.iter() {
                session.get_addr().do_send(ClientMessage { fingerprint: fingerprint.clone(), event: String::from("lobby-closed"), data: json!({}) });
            }
        }
    }
}