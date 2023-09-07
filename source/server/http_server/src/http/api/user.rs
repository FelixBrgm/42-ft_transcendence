use super::error::ApiError;
use crate::http::db::{Database, models::UpdateUser};

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

// home
#[get("/")]
async fn home() -> HttpResponse {
    HttpResponse::Ok().body("welcome home!")
}

// all the users
#[get("/users")]
async fn all(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    match db.get_users() {
		Ok(users) => Ok(HttpResponse::Ok().json(&users)),
		Err(_) => Err(ApiError::InternalServerError)
	}
}

// returns the information of the user sending the request
#[get("/user")]
async fn get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let id = identity.id()?;

    match db.get_user_by_id(id.parse::<i32>()?)? {
		Some(user) => Ok(HttpResponse::Ok().json(&user)),
		None => Err(ApiError::NotFound)
	}
}

// who is allowed to update the user(identity or the game/chat server/frontend-> user
#[post("/user")]
async fn post(update_user: web::Json<UpdateUser>, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {

	let user = update_user.into_inner();

	let msg = format!("User {} {:?} updated succesfully!", user.id, &user.login);
	match db.update_user(&user) 
	{
		Ok(_) => Ok(HttpResponse::Ok().json(msg)),
		Err(_) => Err(ApiError::InternalServerError),
	}
}

// later user_id is retrieved by identity
#[get("/user/rooms")]
async fn rooms_get( db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
	// let user_id = user_id.into_inner();
	// println!("{} rooms, user_id", user_id);

	Ok(HttpResponse::Ok().json("worked"))
}

// #[post("/user/rooms")]

// #[get("/user/rooms/{room_id}]
// #[post("/user/rooms/{room_id}]

// #[get("/user/rooms/{room_id}/messages")]
// #[post("/user/rooms/{room_id}/messages")]
