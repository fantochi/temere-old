use actix::Addr;

use crate::database;

pub mod server;
pub mod routes;
pub mod client;

#[derive(Clone)]
pub struct AppState {
    pub database: Addr<database::DbExecutor>,
    pub server: Addr<server::Server>
}