use actix::{Handler, Message};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::QueryDsl;

use super::DbExecutor;
use crate::{database::session, models, schema};

/* -------------------------------------------------------------------------- */
/*                          GET SESSION FROM DATABASE                         */
/* -------------------------------------------------------------------------- */
pub struct GetSession(pub String);

impl Message for GetSession {
    type Result = Option<models::session::Session>;
}

impl Handler<GetSession> for DbExecutor {
    type Result = Option<models::session::Session>;

    fn handle(&mut self, msg: GetSession, ctx: &mut Self::Context) -> Self::Result {
        use schema::sessions::dsl::*;

        // TODO: Adicionar tratamento de erro ao .get()
        let connection = self.0.get().unwrap();

        let query = sessions.filter(id.eq(msg.0.clone()));

        match query.first::<models::session::Session>(&connection) {
            Ok(e) => Some(e),
            Err(e) => {
                error!("Error on try get session from database: {}", e);
                None
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                      ADD SESSION REGISTRY FOR DATABASE                     */
/* -------------------------------------------------------------------------- */

pub struct RegisterSession {
    pub fingerprint: String,
    pub ip_address: ipnetwork::IpNetwork,
}

impl Message for RegisterSession {
    type Result = ();
}

impl Handler<RegisterSession> for DbExecutor {
    type Result = ();

    fn handle(&mut self, msg: RegisterSession, ctx: &mut Self::Context) -> Self::Result {
        use schema::sessions::dsl::*;

        let connection = self.0.get().unwrap();

        let now = Utc::now().naive_local();

        // TODO: Adicionar update de IP
        let query = diesel::insert_into(sessions)
            .values(models::session::NewSession {
                id: msg.fingerprint,
                address: msg.ip_address,
            })
            .on_conflict(id)
            .do_nothing();

        let result = query.execute(&connection);

        match result {
            Ok(_) => debug!("New session has been registered/updated on database"),
            Err(e) => error!("Error on register/update session on database: {}", e),
        }
    }
}
