use super::error::ApiError;
use actix::prelude::*;
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::game::actor::GameSession;
use crate::game::matchmake::MatchmakingServer;
use crate::game::one_vs_one::OneVsOneServer;
use crate::game::tournament::TournamentServer;
use crate::game::{self, UserId};


#[get("/game/matchmake")]
async fn matchmaking(
    identity: Identity,
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<MatchmakingServer>>,
) -> Result<HttpResponse, ApiError> {
    let client_id = identity.id()?.parse::<usize>()?;

    match ws::start(
        GameSession::new_matchmaking(client_id, server.get_ref().clone()),
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
    if !(size == 2
        || size == 4
        || size == 8
        || size == 16
        || size == 32
        || size == 64
        || size == 128)
    {
        return Err(ApiError::BadRequest(
            "Tournament size must be a power of 2 between 2 and 128".to_string(),
        ));
    }
    match server.get_ref().try_send(game::Create {
        id: client_id,
        size: size,
    }) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/game/connect_tournament/{tournament_id}")]
async fn connect_tournament(
    identity: Identity,
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<TournamentServer>>,
    room_id: web::Path<UserId>,
) -> Result<HttpResponse, ApiError> {
    let client_id = identity.id()?.parse::<usize>()?;

    match ws::start(
        GameSession::new_tournament(client_id, server.get_ref().clone(), room_id.into_inner()),
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
    identity: Identity,
    stream: web::Payload,
    server: web::Data<Addr<OneVsOneServer>>,
    opponent_uid: web::Path<UserId>,
) -> Result<HttpResponse, ApiError> {
    let client_id = identity.id()?.parse::<usize>()?;

    match ws::start(
        GameSession::new_one_vs_one(
            client_id,
            opponent_uid.into_inner(),
            server.get_ref().clone(),
        ),
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
