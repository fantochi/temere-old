use crate::{models, schema};
use actix::{Handler, Message};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use super::DbExecutor;

/* -------------------------------------------------------------------------- */
/*                               GET LOBBY LIST                               */
/* -------------------------------------------------------------------------- */
pub struct GetLobbyList;

impl Message for GetLobbyList {
    type Result = Result<Vec<models::lobby::Lobby>, ()>;
}

impl Handler<GetLobbyList> for DbExecutor {
    type Result = Result<Vec<models::lobby::Lobby>, ()>;

    fn handle(&mut self, _: GetLobbyList, _: &mut Self::Context) -> Self::Result {
        use crate::schema::lobbys::dsl::*;

        let conn = self.0.get();

        match conn {
            Ok(a) => match lobbys.load::<models::lobby::Lobby>(&a) {
                Ok(list) => Ok(list),
                Err(e) => {
                    error!("Error on get list of lobbies");
                    error!("{}", e);
                    Err(())
                }
            },
            Err(e) => {
                error!("Error on get database connection from DbExecutor.0");
                error!("{}", e);
                Err(())
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               GET LOBBY DATA                               */
/* -------------------------------------------------------------------------- */

pub struct GetLobby(pub Uuid);

impl Message for GetLobby {
    type Result = Option<models::lobby::Lobby>;
}

impl Handler<GetLobby> for DbExecutor {
    type Result = Option<models::lobby::Lobby>;

    fn handle(&mut self, msg: GetLobby, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::lobbys::dsl::*;

        let conn = self.0.get();

        match conn {
            Ok(a) => {
                match lobbys
                    .filter(schema::lobbys::id.eq(msg.0))
                    .first::<models::lobby::Lobby>(&a)
                {
                    Ok(lobby) => Some(lobby),
                    Err(e) => {
                        error!("Error on get list of lobbies");
                        error!("{}", e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("Error on get database connection from DbExecutor.0");
                error!("{}", e);
                None
            }
        }
    }
}
