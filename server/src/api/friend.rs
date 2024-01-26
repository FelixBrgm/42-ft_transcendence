use super::error::ApiError;
use crate::db::Database;
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/friend/add/{friend_id}")]
async fn add(
    identity: Identity,
    db: web::Data<Database>,
    friend: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let friend_id = friend.into_inner();

    if !db.check_user(friend_id)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    db.create_friendship(uid, friend_id)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/friend/remove/{friend_id}")]
async fn remove(
    identity: Identity,
    db: web::Data<Database>,
    friend: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let friend_id = friend.into_inner();

    if !db.check_user(friend_id)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    db.remove_friendship(uid, friend_id)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/friend/list")]
async fn list(
    identity: Identity,
    db: web::Data<Database>,
    friend: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let friend_id = friend.into_inner();

    if !db.check_user(friend_id)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    match db.get_all_friendships(uid) {
        Ok(v) => Ok(HttpResponse::Ok().json(&v)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/friend/check/{friend_id}")]
async fn check(
    identity: Identity,
    db: web::Data<Database>,
    friend: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let friend_id = friend.into_inner();

    if !db.check_user(friend_id)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    match db.check_friendship(uid, friend_id)? {
        true => Ok(HttpResponse::Ok().body("User is a friend")),
        false => Ok(HttpResponse::Ok().body("User is not a friend")),
    }
}
