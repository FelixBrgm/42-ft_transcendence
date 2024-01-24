use super::error::ApiError;
use crate::chat::actor::WsActor;
use crate::chat::server::{BlockUser, ChatServer, InsertRoom};
use crate::db::models::NewUser;
use crate::db::Database;
use actix::Addr;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);
static OTHER_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

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

    match ws::start(WsActor::new(uid, server.get_ref().clone()), &req, stream) {
        Ok(ws) => Ok(ws),
        Err(err) => {
            eprintln!("Error during WebSocket handshake: {:?}", err);
            Err(ApiError::InternalServerError)
        }
    }
}

#[get("/chat/{recipient_id}")]
async fn join_chat(
    req: HttpRequest,
    // identity: Identity,
    chat_server: web::Data<Addr<ChatServer>>,
    db: web::Data<Database>,
    user2: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let uid = OTHER_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    let user2 = user2.into_inner();

    if !db.check_user(user2 as i32)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    if db.check_blocked(uid as i32, user2 as i32)? {
        return Err(ApiError::BadRequest("You blocked the user".to_string()));
    }

    if db.check_blocked(user2 as i32, uid as i32)? {
        return Err(ApiError::BadRequest("You are blocked by user".to_string()));
    }

    let rid = db.add_room(user2 as i32, uid as i32)?;

    println!("{}: user1{} user2{}", rid, uid, user2);
    chat_server.do_send(InsertRoom {
        room_id: rid,
        user1: uid,
        user2: user2,
    });

    Ok(HttpResponse::Ok().finish())
}

#[get("/add_friend/{friend_id}")]
async fn create_friendship(
    req: HttpRequest,
    // identity: Identity,
    db: web::Data<Database>,
    user2: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let uid = OTHER_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    let blocked_id = user2.into_inner();

    if !db.check_user(blocked_id as i32)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    db.create_friendship(uid as i32, blocked_id as i32)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/remove_friend/{friend_id}")]
async fn remove_friendship(
    req: HttpRequest,
    // identity: Identity,
    db: web::Data<Database>,
    user2: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let uid = OTHER_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    let blocked_id = user2.into_inner();

    if !db.check_user(blocked_id as i32)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    db.remove_friendship(uid as i32, blocked_id as i32)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/get_friends")]
async fn get_friends(
    req: HttpRequest,
    // identity: Identity,
    db: web::Data<Database>,
    user2: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let uid = OTHER_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    let blocked_id = user2.into_inner();

    if !db.check_user(blocked_id as i32)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    match db.get_all_friendships(uid as i32) {
        Ok(v) => Ok(HttpResponse::Ok().json(&v)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/rooms")]
async fn get_rooms(
    req: HttpRequest,
    // identity: Identity,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let uid = OTHER_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    if !db.check_user(uid as i32)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    match db.get_rooms_by_uid(uid as i32) {
        Ok(v) => Ok(HttpResponse::Ok().json(&v)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/messages/{room_id}")]
async fn get_messages_by_room_id(
    req: HttpRequest,
    // identity: Identity,
    db: web::Data<Database>,
    room_id: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let uid = OTHER_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    let room_id = room_id.into_inner();

    if !db.check_user(uid as i32)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    match db.get_messages_by_room_id(room_id as i32) {
        Ok(v) => Ok(HttpResponse::Ok().json(&v)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/block/{recipient_id}")]
async fn block_user(
    req: HttpRequest,
    // identity: Identity,
    chat_server: web::Data<Addr<ChatServer>>,
    db: web::Data<Database>,
    user2: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let uid = OTHER_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    let blocked_id = user2.into_inner();

    if !db.check_user(blocked_id as i32)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    db.create_blocked(uid as i32, blocked_id as i32)?;

    chat_server.do_send(BlockUser {
        user_id: uid,
        blocked_id,
    });

    Ok(HttpResponse::Ok().finish())
}

#[get("/unblock/{recipient_id}")]
async fn unblock_user(
    req: HttpRequest,
    // identity: Identity,
    chat_server: web::Data<Addr<ChatServer>>,
    db: web::Data<Database>,
    user2: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let uid = OTHER_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    let blocked_id = user2.into_inner();

    if !db.check_user(blocked_id as i32)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    db.remove_blocked(uid as i32, blocked_id as i32)?;

    Ok(HttpResponse::Ok().finish())
}
