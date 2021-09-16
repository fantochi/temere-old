#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

mod app;
pub mod database;
pub mod models;
mod routes;
pub mod schema;

use actix::{Actor, SyncArbiter};
use actix_cors::Cors;
use actix_web::{
    http,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file(
            std::env::var("SSL_KEY_PATH").expect("SSL_KEY_PATH must be set"),
            SslFiletype::PEM,
        )
        .unwrap();

    builder
        .set_certificate_chain_file(
            std::env::var("SSL_CERT_PATH").expect("SSL_CERT_PATH must be set"),
        )
        .unwrap();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let database_pool = database::new_pool().unwrap();
    let database = SyncArbiter::start(1, move || database::DbExecutor::new(database_pool.clone()));
    let server = app::server::Server::new(database.clone()).start();

    HttpServer::new(move || {
        let state = app::AppState {
            database: database.clone(),
            server: server.clone(),
        };

        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .route("/", web::get().to(|| HttpResponse::Ok()))
            .configure(routes::config)
            .app_data(Data::new(state.clone()))
            //.wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
    })
    .bind_openssl("51.79.25.227:8080", builder)?
    .run()
    .await
}
