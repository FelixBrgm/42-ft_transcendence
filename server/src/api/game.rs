use super::error::ApiError;
use actix::prelude::*;
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;

use crate::db::Database;
use crate::game::actor::GameSession;
use crate::game::matchmake::MatchmakingServer;
use crate::game::one_vs_one::OneVsOneServer;
use crate::game::tournament::TournamentServer;
use crate::game::{self, UserId};

#[derive(Deserialize)]
struct Info {
    id: usize,
    token: String,
}

#[get("/game/matchmake/")]
async fn matchmaking(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<MatchmakingServer>>,
    info: web::Query<Info>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    if !db.check_user_token(info.id as i32, &info.token)? {
        return Err(ApiError::Unauthorized);
    }

    match ws::start(
        GameSession::new_matchmaking(info.id, server.get_ref().clone()),
        &req,
        stream,
    ) {
        Ok(ws) => Ok(ws),
        Err(err) => {
            eprintln!("Error during WebSocket handshake: {:?}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/game/create_tournament/{size}")]
async fn create_tournament(
    identity: Identity,
    server: web::Data<Addr<TournamentServer>>,
    size: web::Path<u8>,
) -> Result<HttpResponse, ApiError> {
    let client_id = identity.id()?.parse::<usize>()?;

    let size = size.into_inner();
    if !(size == 4
        || size == 8
        || size == 16
        || size == 32
        || size == 64
        || size == 128)
    {
        return Err(ApiError::BadRequest(
            "Tournament size must be a power of 2 between 4 and 128".to_string(),
        ));
    }
    match server.get_ref().try_send(game::Create {
        id: client_id,
        size: size,
    }) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(ApiError::BadRequest(format!("Error: You're hosting a ongiong Tournament"))),
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

#[get("/game/connect_tournament/{tournament_id}")]
async fn connect_tournament(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<TournamentServer>>,
    room_id: web::Path<UserId>,
    // info: web::Query<Info>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    // if !db.check_user_token(info.id as i32, &info.token)? {
    //     return Err(ApiError::Unauthorized);
    // }

    let uid = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    match ws::start(
        GameSession::new_tournament(uid, server.get_ref().clone(), room_id.into_inner()),
        &req,
        stream,
    ) {
        Ok(ws) => Ok(ws),
        Err(err) => {
            eprintln!("Error during WebSocket handshake: {:?}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/game/one_vs_one/{opponent_uid}")]
async fn one_vs_one(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<OneVsOneServer>>,
    opponent_uid: web::Path<UserId>,
    info: web::Query<Info>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
	let opponent_uid = opponent_uid.into_inner();

    if !db.check_user_token(info.id as i32, &info.token)? {
        return Err(ApiError::Unauthorized);
    }

	if !db.check_user(opponent_uid as i32)? {
		return Err(ApiError::BadRequest("The requested User doesn't exits!".to_string()));
	}

    match ws::start(
        GameSession::new_one_vs_one(info.id, opponent_uid, server.get_ref().clone()),
        &req,
        stream,
    ) {
        Ok(ws) => Ok(ws),
        Err(err) => {
            eprintln!("Error during WebSocket handshake: {:?}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/game/list/{uid}")]
async fn list(
    identity: Identity,
    req: HttpRequest,
    uid: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let uid = uid.into_inner();

    match db.get_games_by_uid(uid) {
        Ok(game) => Ok(HttpResponse::Ok().json(game)),
        Err(_) => Err(ApiError::BadRequest("User was not found".to_string())),
    }
}
