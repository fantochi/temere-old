use actix::Addr;

mod database;
mod app;

pub struct AppState {
    database: Addr<database::DbExecutor>,
    server: Addr<app::server::Server>
}