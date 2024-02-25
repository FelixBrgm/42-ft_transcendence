use super::error::ApiError;
use crate::db::Database;
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};

#[get("/api/friend/{friend_id}")]
async fn toggle(
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
        false => {
            db.create_friendship(uid, friend_id)?;
        }
        true => {
            db.remove_friendship(uid, friend_id)?;
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[get("/api/friend/list/{uid}")]
async fn list(
    _: Identity,
    uid: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let uid = uid.into_inner();

    match db.get_all_friendships(uid) {
        Ok(v) => Ok(HttpResponse::Ok().json(&v)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/api/friend/check/{friend_id}")]
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

    Ok(HttpResponse::Ok().json(db.check_friendship(uid, friend_id)?))
}
