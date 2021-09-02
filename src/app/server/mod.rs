pub mod chat;
pub mod lobby;

use std::collections::HashMap;
use actix::{Actor, ActorFuture, Addr, Context, Handler, Message, WrapFuture};
use actix::{fut,  ContextFutureSpawner};

use uuid::Uuid;

use crate::database::{self, DbExecutor};

pub struct Server {
    lobbys: HashMap<Uuid, Addr<lobby::Lobby>>,
    watcher: String,
}

impl Server {
    pub fn new() -> Self {
        Self {
            lobbys: HashMap::new(),
            watcher: String::default()
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;    
}

/* -------------------------------------------------------------------------- */
/*                                   ACTIONS                                  */
/* -------------------------------------------------------------------------- */

// Get Actor Addr from Lobby
pub struct GetLobbyAddr{
    pub id: Uuid
}

impl Message for GetLobbyAddr {
    type Result = Option<Addr<lobby::Lobby>>;
}

impl Handler<GetLobbyAddr> for Server {
    type Result = Option<Addr<lobby::Lobby>>;

    fn handle(&mut self, msg: GetLobbyAddr, _ctx: &mut Self::Context) -> Self::Result {
        
        let addr= self.lobbys.get(&msg.id);

        match addr {
            Some(a) => Some(a.clone()),
            None => None
        }
    }
}

// Scan lobbies from database
// TODO: Make thread to periodical scan, to open or close lobbies according to the database
pub struct LoadLobbies(pub Addr<DbExecutor>);

impl Message for LoadLobbies {
    type Result = ();    
}

impl Handler<LoadLobbies> for Server {
    type Result = ();

    fn handle(&mut self, msg: LoadLobbies, ctx: &mut Self::Context) -> Self::Result {
        info!("Loading lobbies...");
        let db_executor = msg.0.clone();        
        db_executor.send(database::lobby::GetLobbyList)
            .into_actor(self)
            .then(|res, server, _ctx| {
                if let Ok(Ok(list)) = res {
                    for lobby in list {
                        if let Some(lb) = server.lobbys.get_mut(&lobby.id) {
                            lb.do_send(lobby::Enabled(lobby.enabled));
                        } else {
                            server.lobbys.insert(lobby.id, lobby::Lobby::new(lobby.enabled).start());
                        }
                    }          
                }
                fut::ready(())
            })
            .wait(ctx);

        info!("Lobbies OK!");
    }
}

