use super::error::ApiError;
use crate::http::db::{models::NewChatRoom, models::UpdateChatRoom, Database};

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

// returns the information of the user sending the request
#[get("/rooms")]
async fn all(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    match db.get_rooms() {
        Ok(rooms) => Ok(HttpResponse::Ok().json(&rooms)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/room/{id}")]
async fn get(room_id: web::Path<i32>, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let id: i32 = room_id.into_inner();

    match db.get_room_by_id(id)? {
        Some(room) => Ok(HttpResponse::Ok().json(&room)),
        None => Err(ApiError::NotFound),
    }
}

// #[get("/room/{id}/messages")]

// -> add owner
// -> add connections between them
#[post("/room")]
async fn post(
    new_room: web::Json<NewChatRoom>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let room = new_room.into_inner();

    let msg = format!("Room {} added succesfully!", room.name);
    match db.add_room(&room) {
        Ok(_) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[post("/room/{room_id}/add/{user_id}")]
async fn add_user(
    ids: web::Path<(i32, i32)>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let room_id = ids.0;
    let user_id = ids.1;

    let msg = format!("Room {} added User {} succesfully!", room_id, user_id);
    match db.add_connection(user_id, room_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[post("/room/{room_id}/rem/{user_id}")]
async fn rem_user(
    ids: web::Path<(i32, i32)>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let room_id = ids.0;
    let user_id = ids.1;

    let msg = format!("Room {} rem User {} succesfully!", room_id, user_id);
    match db.rem_connection(user_id, room_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

// #[post("/room/{room_id}/{user_id}")]
// async fn rem_user(
//     room_id: web::Path<i32>,
//     user_id: web::Path<i32>,
//     db: web::Data<Database>,
// ) -> Result<HttpResponse, ApiError> {
// }

// -------------------- CONNECTIONS -----------------------

// #[post("/room/{id}/rem_user")]
// #[post("/room/{id}/change_topicr")]
// #[post("/room/{id}/change_name")]
// #[post("/room/{id}/change_owner")]
// #[post("/room/{id}/is_public/{bool}]

// #[post("rooms/{id}/change_password")]

// #[post("/room/{id}/messages")]
