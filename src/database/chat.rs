use actix::{Handler, Message};
use diesel::{RunQueryDsl, Table};
use diesel::prelude::*;
use uuid::Uuid;

use super::DbExecutor;
use crate::{models, schema};

/* -------------------------------------------------------------------------- */
/*                         CREATE NEW CHAT ON DATABASE                        */
/* -------------------------------------------------------------------------- */
pub struct RegisterChat(pub Uuid);

impl Message for RegisterChat {
    type Result = Result<models::chat::Chat, diesel::result::Error>;
}

impl Handler<RegisterChat> for DbExecutor {
    type Result = Result<models::chat::Chat, diesel::result::Error>;

    fn handle(&mut self, msg: RegisterChat, ctx: &mut Self::Context) -> Self::Result {
        use schema::chats::dsl::*;

        let connection = self.0.get().unwrap();

        let query = diesel::insert_into(chats)
            .values(lobby_id.eq(msg.0.clone()));
        
        query.get_result::<models::chat::Chat>(&connection)
    }
}