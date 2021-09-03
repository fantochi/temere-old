#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod app;
pub mod models;
pub mod database;

use actix::{Actor, SyncArbiter};
use actix_web::{App, HttpResponse, HttpServer, web::{self, Data}, http};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let database_pool = database::new_pool().unwrap();
    let database = SyncArbiter::start(1, move | | {database::DbExecutor::new(database_pool.clone())});
    let server = app::server::Server::new().start();
    let _ = server.send(app::server::LoadLobbies(database.clone())).await;

    //cron(server.clone(),database.clone());

    HttpServer::new(move || {
        let state = app::AppState {
            database: database.clone(),
            server: server.clone()
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
            .configure(app::routes::config)
            .app_data(Data::new(state.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
    })
    .bind("192.168.50.219:8080")?
    .run()
    .await
}

// TODO: This not wokr
// async fn cron(server_addr: Addr<app::server::Server>, database_addr: Addr<database::DbExecutor>) {
//     loop{
//         info!("TO AQUIIII");
//         server_addr.do_send(app::server::LoadLobbies(database_addr.clone()));
//         actix_rt::time::sleep(std::time::Duration::from_secs(5)).await
//     }
// }