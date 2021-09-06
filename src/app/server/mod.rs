pub mod chat;
pub mod lobby;

use std::collections::HashMap;
use actix::{Actor, ActorFuture, Addr, Context, Handler, Message, WrapFuture};
use actix::{fut,  ContextFutureSpawner};
use uuid::Uuid;

use crate::database;

/* -------------------------------------------------------------------------- */
/*                                   SERVER                                   */
/* -------------------------------------------------------------------------- */

// The server is responsible to load, update and storage all lobbies from database
pub struct Server {
    db_executor: Addr<database::DbExecutor>,
    lobbies: HashMap<Uuid, Addr<lobby::Lobby>>
}

impl Server {
    pub fn new(db_executor: Addr<database::DbExecutor>) -> Self {
        Self {
            db_executor,
            lobbies: HashMap::new()
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        // Get lobby list from database and load on self.lobbies
        self.db_executor.send(database::lobby::GetLobbyList)
            .into_actor(self)
            .then(|res, server, _| {
                if let Ok(Ok(list)) = res {
                    for lb in list {
                        server.lobbies.insert(lb.id.clone(), lobby::Lobby::new(lb, server.db_executor.clone()).start());
                    }          
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}


/* ----------------- HANDLER TO GET LIST OF LOBBIES AVAIBLE ----------------- */

// When a new connection require the lobby actor addr, this is responsible to find and response Option<actix:Addr<lobby::Lobby>>
pub struct GetLobbyAddr{
    pub id: Uuid
}

impl Message for GetLobbyAddr {
    type Result = Option<Addr<lobby::Lobby>>;
}

impl Handler<GetLobbyAddr> for Server {
    type Result = Option<Addr<lobby::Lobby>>;

    fn handle(&mut self, msg: GetLobbyAddr, _ctx: &mut Self::Context) -> Self::Result {
        
        let addr= self.lobbies.get(&msg.id);

        match addr {
            Some(a) => Some(a.clone()),
            None => None
        }
    }
}