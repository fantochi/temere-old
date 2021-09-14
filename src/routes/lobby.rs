use actix_web::{
    get,
    web::{self, Data, Payload},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

use crate::{
    app::{client, server},
    database,
};

use crate::app::AppState;

// GET LIST OF ALL LOBBIES ENABLED
#[get("/lobby")]
pub async fn list(
    req: HttpRequest,
    stream: Payload,
    state: Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let db_result = state.database.send(database::GetLobbyList).await;

    match db_result {
        Ok(result) => match result {
            Ok(lobbys) => Ok(HttpResponse::Ok().body(json!(lobbys))),
            Err(e) => Err(actix_web::error::ErrorInternalServerError("deu ruim")), // TODO ADD ERROR TO RESPONSE
        },
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[get("/lobby/{lobby_id}")]
pub async fn get_lobby(
    req: HttpRequest,
    stream: Payload,
    state: Data<AppState>,
    data: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let lobby_id = data.parse::<Uuid>().unwrap();

    let state = state.get_ref().clone();

    let db_result = state.database.send(database::GetLobby(lobby_id)).await;

    match db_result {
        Ok(result) => Ok(HttpResponse::Ok().body(json!(result))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError("Error on quary")),
    }
}

#[get("/ws/{lobby_id}/{fingerprint}")]
pub async fn join(
    req: HttpRequest,
    stream: Payload,
    state: Data<AppState>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, actix_web::Error> {
    let state = state.get_ref().clone();
    let data = data.clone();

    let lobby_id = data.0.parse::<Uuid>();
    let fingerprint = data.1;

    if let Err(_) = lobby_id {
        return Err(actix_web::error::ErrorBadRequest("invalid lobby id!"));
    }

    let lobby_id = lobby_id.unwrap();

    let lobby_addr = state
        .server
        .send(server::GetLobbyAddr {
            id: lobby_id.clone(),
        })
        .await
        .unwrap();

    match lobby_addr {
        None => return Err(actix_web::error::ErrorBadRequest("Lobby id not exists!")),
        Some(addr) => {
            let client = client::Client::new(fingerprint.clone(), addr);
            let resp = actix_web_actors::ws::start(client, &req, stream)?;

            if let Some(val) = req.peer_addr() {
                state.database.do_send(database::RegisterSession {
                    fingerprint: fingerprint.clone(),
                    ip_address: ipnetwork::IpNetwork::from(val.ip()),
                });
            };

            return Ok(resp);
        }
    }
}
