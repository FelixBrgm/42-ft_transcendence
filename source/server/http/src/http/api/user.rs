use super::error::ApiError;
use crate::http::db::{models::UpdateUser, Database};

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

// home
#[get("/")]
async fn home() -> HttpResponse {
    HttpResponse::Ok().body("welcome home!")
}

//  clear
#[get("/clear")]
async fn clear(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    match db.clear_tables() {
        Ok(users) => Ok(HttpResponse::Ok().json("database has cleared all tables!")),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

// all the users
#[get("/users")]
async fn all(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    match db.get_users() {
        Ok(users) => Ok(HttpResponse::Ok().json(&users)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

// returns the information of the user sending the request
#[get("/user")]
async fn get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let id = identity.id()?;

    match db.get_user_by_id(id.parse::<i32>()?)? {
        Some(user) => Ok(HttpResponse::Ok().json(&user)),
        None => Err(ApiError::NotFound),
    }
}

// who is allowed to update the user(identity or the game/chat server/frontend-> user
#[post("/user")]
async fn post(
    identity: Identity,
    update_user: web::Json<UpdateUser>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user = update_user.into_inner();
    let uid = identity.id()?.parse::<i32>()?;

    let msg = format!("User {} updated succesfully!", uid);
    match db.update_user(&user, uid) {
        Ok(_) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/user/room")]
async fn rooms(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    match db.get_user_connections(uid) {
        Ok(rooms) => Ok(HttpResponse::Ok().json(rooms)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

// later user_id is retrieved by identity
// #[get("/user/rooms")]
// async fn rooms_get(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
// }

// #[post("/user/rooms")]

// #[get("/user/rooms/{room_id}]
// #[post("/user/rooms/{room_id}]

// #[get("/user/rooms/{room_id}/messages")]
// #[post("/user/rooms/{room_id}/messages")]
