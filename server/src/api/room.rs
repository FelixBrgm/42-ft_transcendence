use super::error::ApiError;
use crate::db::models::{NewChatRoom, UpdateChatRoom};
use crate::db::Database;

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

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
