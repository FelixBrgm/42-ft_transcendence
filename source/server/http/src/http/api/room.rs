use super::error::ApiError;
use crate::http::db::models::{NewChatRoom, NewMessage, UpdateChatRoom};
use crate::http::db::Database;
use crate::http::RoomSocket;

use tokio::sync::mpsc::Sender;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

// call them all rooms
// ids after rooms/

#[get("/rooms")]
async fn all(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    match db.get_rooms() {
        Ok(rooms) => Ok(HttpResponse::Ok().json(&rooms)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/room/{room_id}")]
async fn get(room_id: web::Path<i32>, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let rid: i32 = room_id.into_inner();

    match db.get_room_by_id(rid)? {
        Some(room) => Ok(HttpResponse::Ok().json(&room)),
        None => Err(ApiError::NotFound),
    }
}

#[get("/room/list/{room_id}")]
async fn list(room_id: web::Path<i32>, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let rid: i32 = room_id.into_inner();

    match db.get_room_connections(rid) {
        Ok(rooms) => Ok(HttpResponse::Ok().json(&rooms)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

// how to implement this whith the chat server
#[get("/room/messages/{room_id}")]
async fn messages(
    room_id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let rid: i32 = room_id.into_inner();

    match db.get_messages_by_room_id(rid) {
        Ok(rooms) => Ok(HttpResponse::Ok().json(&rooms)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[post("/room/create")]
async fn create(
    identity: Identity,
    new_room: web::Json<NewChatRoom>,
	room_update_sender: web::Data<Sender<RoomSocket>>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let room = new_room.into_inner();
    let uid = identity.id()?.parse::<i32>()?;

    match db.create_room(room, uid) {
        Ok(rid) => {
			Ok(HttpResponse::Ok().json(format!("Room {} added succesfully!", rid)))
		}
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[post("/room/create/personal/{user_id}")]
async fn personal(
    identity: Identity,
    user_id: web::Path<i32>,
	room_update_sender: web::Data<Sender<RoomSocket>>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let owner_id = identity.id()?.parse::<i32>()?;
    let partner_id = user_id.into_inner();

    match db.create_personal_room(owner_id, partner_id) {
        Ok(rid) => Ok(HttpResponse::Ok().json(format!("personal Room {} added succesfully!", rid))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[post("/room/update")]
async fn update(
    identity: Identity,
    update_room: web::Json<UpdateChatRoom>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;

    match db.update_room(&update_room, uid) {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(format!("Room {} updated succesfully!", update_room.id)))
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[post("/room/join/{room_id}")]
async fn join(
    identity: Identity,
    id: web::Path<i32>,
	room_update_sender: web::Data<Sender<RoomSocket>>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let room_id = id.into_inner();
    let user_id = identity.id()?.parse::<i32>()?;

    let msg = format!("User {} is in Room {}!", user_id, room_id);
    match db.add_connection(user_id, room_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/room/part/{room_id}")]
async fn part(
    identity: Identity,
    id: web::Path<i32>,
	room_update_sender: web::Data<Sender<RoomSocket>>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let room_id = id.into_inner();
    let user_id = identity.id()?.parse::<i32>()?;

    let msg = format!("User {} isn't int Room {}!", user_id, room_id);
    match db.part_room(user_id, room_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
