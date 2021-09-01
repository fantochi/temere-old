use actix::{Handler, Message};
use diesel::{QueryDsl, RunQueryDsl};
use crate::{app::routes::lobby, models};

use super::DbExecutor;

pub struct GetLobbyList;

impl Message for GetLobbyList {
    type Result =  Result<Vec<models::lobby::Lobby>, ()>;
}

impl Handler<GetLobbyList> for DbExecutor {
    type Result = Result<Vec<models::lobby::Lobby>, ()>;

    fn handle(&mut self, _: GetLobbyList, _: &mut Self::Context) -> Self::Result {
        
        use crate::schema::lobbys::dsl::*;

        let conn = self.0.get();

        match conn {
            Ok(a) => {
                
                match lobbys.load::<models::lobby::Lobby>(&a) {
                    Ok(list) => {
                        Ok(list)
                    },
                    Err(e) => {
                        error!("Error on get list of lobbies");
                        error!("{}", e);
                        Err(())
                    }
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