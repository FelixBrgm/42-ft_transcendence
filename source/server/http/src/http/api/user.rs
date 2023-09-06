use super::error::ApiError;
use crate::http::db::Database;

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

// default url
#[get("/")]
async fn home() -> HttpResponse {
    HttpResponse::Ok().body("welcome home!")
}

// returns the information of the user sending the request
#[get("/user")]
async fn user_get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let id = identity.id()?;

    match db.get_user_by_id(id.parse::<i32>()?)? {
		Some(user) => Ok(HttpResponse::Ok().json(&user)),
		None => Ok(HttpResponse::NotFound().body("User not in database"))
	}
}

// #[get("/user/rooms")]
// #[post("/user/rooms")]

// #[get("/user/rooms/{room_id}/messages")]
// #[post("/user/rooms/{room_id}/messages")]

use serde::Deserialize;
#[derive(Deserialize)]
struct UserId {
    id: String, // Assuming the id is a string in the path
}

#[get("/user/{id}")]
async fn user_get_test(
    path: web::Path<UserId>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {

    let id = path.id.parse::<i32>()?;

    if let Ok(user) = db.get_user_by_id(id) {
        return Ok(HttpResponse::Ok().json(&user));
    }

    Err(ApiError::NotFound)
}
