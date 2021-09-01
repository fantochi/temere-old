use actix_web::{HttpRequest, HttpResponse, get, web::{self, Data, Payload}};
use serde::Deserialize;

use crate::database;

use super::super::AppState;

#[get("/lobby")]
pub async fn list(
    req: HttpRequest,
    stream: Payload,
    state: Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    
    let state = state.get_ref().clone();

    let db_result = state.database.send(database::lobby::GetLobbyList).await;

    match db_result {
        Ok(result) => Ok(HttpResponse::Ok().body(json!(result))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError("Error on quary"))
    }
}

#[derive(Deserialize)]
pub struct Credentials {
    fingerprint: String
}

#[get("/lobby/{lobby_id}/{fingerprint}")]
pub async fn join(
    req: HttpRequest,
    stream: Payload,
    state: Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {

    let state = state.get_ref().clone();
    
    

    // let ws = Client::new(
    //     srv.get_ref().clone(),
    // );

    // let resp = ws::start(ws, &req, stream)?;
    // Ok(resp)

    todo!()
}