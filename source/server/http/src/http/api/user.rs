use super::error::ApiError;
use crate::http::db::Database;

use actix_web::{get, post, web, HttpResponse};
use actix_identity::Identity;
use anyhow::Result;

// default url
#[get("/")]
async fn home() -> HttpResponse
{
	HttpResponse::Ok().body("welcome home!")
}

// returns the information of the user sending the request
#[get("/user")]
async fn user_get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    
	let id = identity.id()?;
    let user = db.get_user_by_id(id.parse::<i32>()?)?;

    Ok(HttpResponse::Ok().json(&user))
}

// #[get("/user/rooms")]
// #[post("/user/rooms")]

// #[get("/user/rooms/{room_id}/messages")]
// #[post("/user/rooms/{room_id}/messages")]


