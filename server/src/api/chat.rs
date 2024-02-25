use actix::Addr;
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;

use super::error::ApiError;
use crate::chat::actor::WsActor;
use crate::chat::server::{ChatServer, InsertRoom};
use crate::db::Database;

#[derive(Deserialize)]
struct Info {
    id: i32,
    token: String,
}

#[get("/ws")]
async fn server(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
    info: web::Query<Info>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    if !db.check_user_token(info.id, &info.token)? {
        return Err(ApiError::Unauthorized);
    }

    match ws::start(
        WsActor::new(info.id, server.get_ref().clone()),
        &req,
        stream,
    ) {
        Ok(ws) => Ok(ws),
        Err(err) => {
            eprintln!("Error during WebSocket handshake: {:?}", err);
            Err(ApiError::InternalServerError)
        }
    }
}

#[get("/chat/{recipient_id}")]
async fn join_chat(
    identity: Identity,
    chat_server: web::Data<Addr<ChatServer>>,
    db: web::Data<Database>,
    user2: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let user2 = user2.into_inner();

    if !db.check_user(user2)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    if db.check_blocked(uid, user2)? {
        return Err(ApiError::BadRequest("You blocked the user".to_string()));
    }

    let rid = match db.check_room_by_user(uid, user2)? {
        Some(rid) => rid,
        None => db.add_room(user2, uid)?,
    };

    chat_server.do_send(InsertRoom {
        room_id: rid,
        user1: uid,
        user2: user2,
    });

    Ok(HttpResponse::Ok().json(rid))
}

#[get("/rooms")]
async fn get_rooms(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;

    if !db.check_user(uid)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    match db.get_rooms_by_uid(uid) {
        Ok(v) => Ok(HttpResponse::Ok().json(&v)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/messages/{room_id}")]
async fn get_messages_by_room_id(
    identity: Identity,
    db: web::Data<Database>,
    room_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let room_id = room_id.into_inner();

    if !db.check_user(uid)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    match db.get_messages_by_room_id(room_id) {
        Ok(v) => {
            Ok(HttpResponse::Ok().json(&v))
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}
