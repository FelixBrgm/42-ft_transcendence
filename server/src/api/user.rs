use super::error::ApiError;
use crate::db::{models::UpdateUser, Database};

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

// home
#[get("/")]
async fn home() -> HttpResponse {
    HttpResponse::Ok().body("welcome home!")
}

#[get("/user")]
async fn get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let id = identity.id()?;

    match db.get_user_by_id(id.parse::<i32>()?)? {
        Some(user) => Ok(HttpResponse::Ok().json(&user)),
        None => Err(ApiError::NotFound),
    }
}

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

#[get("/user/user_id")]
async fn find(
    _: Identity,
    user: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();

    match db.get_user_by_id(user) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
