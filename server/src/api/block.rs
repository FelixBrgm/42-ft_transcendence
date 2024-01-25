use super::error::ApiError;
use crate::chat::server::{BlockUser, ChatServer};
use crate::db::Database;
use actix::Addr;
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/block/{blocked_id}")]
async fn add(
    req: HttpRequest,
    identity: Identity,
    chat_server: web::Data<Addr<ChatServer>>,
    db: web::Data<Database>,
    blocked: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let blocked_id = blocked.into_inner();

    if !db.check_user(blocked_id)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    db.create_blocked(uid, blocked_id)?;

    chat_server.do_send(BlockUser {
        user_id: uid,
        blocked_id,
    });

    Ok(HttpResponse::Ok().finish())
}

#[get("/block/check/{blocked_id}")]
async fn remove(
    req: HttpRequest,
    identity: Identity,
    db: web::Data<Database>,
    blocked: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let blocked_id = blocked.into_inner();

    if !db.check_user(blocked_id)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    db.remove_blocked(uid, blocked_id)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/block/check/{blocked_id}")]
async fn check(
    req: HttpRequest,
    identity: Identity,
    db: web::Data<Database>,
    blocked: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let uid = identity.id()?.parse::<i32>()?;
    let blocked_id = blocked.into_inner();

    if !db.check_user(blocked_id)? {
        return Err(ApiError::BadRequest(
            "Requested user doesn't exist".to_string(),
        ));
    }

    match db.check_blocked(uid, blocked_id)? {
		true => Ok(HttpResponse::Ok().body("User is blocked")),
        false => Ok(HttpResponse::Ok().body("User is not blocked")),
	}
}

