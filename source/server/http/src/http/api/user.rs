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

#[post("/user")]
async fn user_post(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
   
	let id = identity.id()?;

    // let result = db.set_user();

    // match result {
    //     Ok(_) => Ok(HttpResponse::Ok().finish()),
    //     Err(_) => Err(ApiError::InternalServerError),
    // }

    Ok(HttpResponse::Ok().finish())
}
