use super::error::ApiError;
use crate::http::db::Database;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

// returns the information of the user sending the request
#[get("/user")]
async fn user_get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let Ok(id) = identity.id() else {
		return Err(ApiError::Unauthorized)
	};

    let user = db.get_user_by_id(id.parse::<i32>()?)?;
    Ok(HttpResponse::Ok().json(&user))
}

#[post("/user")]
async fn user_post(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let Ok(id) = identity.id() else {
		return Err(ApiError::Unauthorized)
	};

    // let result = db.set_user();

    // match result {
    //     Ok(_) => Ok(HttpResponse::Ok().finish()),
    //     Err(_) => Err(ApiError::InternalServerError),
    // }

    Ok(HttpResponse::Ok().finish())
}
