#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

mod app;
mod database;
mod lib;

use actix::{Actor, SyncArbiter};
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    env_logger::init();

    let server = app::server::Server::new().start();
    let database = SyncArbiter::start(num_cpus::get(), | | {database::DbExecutor::new()?});

    HttpServer::new(move || {

        let state = lib::AppState {
            database,
            server
        };

        App::new()
            .route("/", web::get().to(|| HttpResponse::Ok()))
            .data(state)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}