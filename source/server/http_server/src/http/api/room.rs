use super::error::ApiError;
use crate::http::db::{Database};

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

// returns the information of the user sending the request
#[get("/rooms")]
async fn all(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
	match db.get_rooms() {
		Ok(rooms) => Ok(HttpResponse::Ok().json(&rooms)),
		Err(_) => Err(ApiError::InternalServerError)
	}
}
