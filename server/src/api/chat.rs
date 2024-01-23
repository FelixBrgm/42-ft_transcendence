use super::error::ApiError;
use crate::chat::{actor::WsActor, server::ChatServer};
use crate::db::Database;
use actix::Addr;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

// TODO: need to load all the past messages in the room
#[get("/ws")]
async fn server(
    req: HttpRequest,
    // identity: Identity,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let uid = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    // let uid = identity.id()?.parse::<usize>()?;

    match ws::start(
        WsActor::new(uid, server.get_ref().clone()),
        &req,
        stream,
    ) {
        Ok(ws) => Ok(ws),
        Err(err) => {
            eprintln!("Error during WebSocket handshake: {:?}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/chat/{recipient_id}")]
async fn join_chat