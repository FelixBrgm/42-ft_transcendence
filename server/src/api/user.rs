use super::error::ApiError;
use crate::db::{models::UpdateUser, Database};

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

#[get("/api/user")]
async fn get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let id = identity.id()?;

    match db.get_user_by_id(id.parse::<i32>()?)? {
        Some(user) => Ok(HttpResponse::Ok().json(&user)),
        None => Err(ApiError::NotFound),
    }
}

#[post("/api/user")]
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

#[get("/api/users")]
async fn list(
	_: Identity,
	db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {

	match db.get_all_users() {
		Ok(users) => Ok(HttpResponse::Ok().json(users)),
		Err(_) => Err(ApiError::InternalServerError),
	}
}

#[get("/api/user/{user_id}")]
async fn find(
    _: Identity,
    user_id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user = user_id.into_inner();

    match db.get_user_by_id(user) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

