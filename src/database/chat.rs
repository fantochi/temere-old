use actix::{Handler, Message};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{query_dsl, RunQueryDsl, Table};
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

        let query = diesel::insert_into(chats).values(lobby_id.eq(msg.0.clone()));

        query.get_result::<models::chat::Chat>(&connection)
    }
}

/* -------------------------------------------------------------------------- */
/*                           UPDATE CHAT ON DATABASE                          */
/* -------------------------------------------------------------------------- */
pub struct CloseChat {
    pub chat_id: Uuid,
    pub message_counter: i32,
}

impl Message for CloseChat {
    type Result = ();
}

impl Handler<CloseChat> for DbExecutor {
    type Result = ();

    fn handle(&mut self, msg: CloseChat, ctx: &mut Self::Context) -> Self::Result {
        use schema::chats::dsl::*;

        let connection = self.0.get().unwrap();
        let now = Utc::now().naive_local();

        let target = chats.filter(id.eq(msg.chat_id.clone()));

        let query = diesel::update(target).set(models::chat::UpdateChat {
            message_counter: msg.message_counter,
            status: "closed".to_string(),
            updated_at: now,
        });

        query.execute(&connection);
    }
}
