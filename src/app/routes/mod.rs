pub mod lobby;

use actix_web::{web, middleware, App, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Routes config called");
    cfg.service(lobby::join);
    cfg.service(lobby::list);
    cfg.service(lobby::get_lobby);
}