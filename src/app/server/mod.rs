pub mod chat;
pub mod lobby;

use std::collections::HashMap;
use actix::{Actor, ActorFuture, Addr, Context, Handler, Message, WrapFuture};
use actix::{fut,  ContextFutureSpawner};

use uuid::Uuid;

use crate::database::{self, DbExecutor};

pub struct Server {
    db_executor: Addr<DbExecutor>,
    lobbys: HashMap<Uuid, Addr<lobby::Lobby>>
}

impl Server {
    pub fn new(db_executor: Addr<DbExecutor>) -> Self {
        Self {
            db_executor,
            lobbys: HashMap::new()
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.db_executor.send(database::lobby::GetLobbyList)
            .into_actor(self)
            .then(|res, server, _ctx| {
                if let Ok(Ok(list)) = res {
                    for lobby in list {
                        server.lobbys.insert(lobby.id.clone(), lobby::Lobby::new(lobby.id.clone(), server.db_executor.clone()).start());
                    }          
                }
                fut::ready(())
            })
            .wait(ctx);
    }
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