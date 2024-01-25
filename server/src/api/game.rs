use super::error::ApiError;
use actix::prelude::*;

use crate::db::Database;

use crate::game::matchmake::MatchmakingServer;
use crate::game::one_vs_one::OneVsOneServer;
use crate::game::tournament::TournamentServer;
use crate::game::{self, UserId};

use actix::{Actor, Addr, StreamHandler};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Debug)]
pub enum GameMode {
    OneVsOne(Addr<OneVsOneServer>),
    Matchmaking(Addr<MatchmakingServer>),
    Tournament(Addr<TournamentServer>),
}

pub struct GameSession {
    id: usize,
    hb: Instant,
    game_mode: GameMode,
    room_id: Option<UserId>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Stop {
    pub id: usize,
}

impl GameSession {
    fn new_one_vs_one(
        id: usize,
        opponent_uid: usize,
        one_vs_one_server: Addr<OneVsOneServer>,
    ) -> Self {
        GameSession {
            id,
            game_mode: GameMode::OneVsOne(one_vs_one_server),
            hb: Instant::now(),
            room_id: Some(opponent_uid),
        }
    }

    fn new_matchmaking(id: usize, matchmaking_server: Addr<MatchmakingServer>) -> Self {
        GameSession {
            id,
            game_mode: GameMode::Matchmaking(matchmaking_server),
            hb: Instant::now(),
            room_id: None,
        }
    }

    fn new_tournament(
        id: usize,
        tournament_server: Addr<TournamentServer>,
        room_id: UserId,
    ) -> Self {
        GameSession {
            id,
            game_mode: GameMode::Tournament(tournament_server),
            hb: Instant::now(),
            room_id: Some(room_id),
        }
    }
}

impl GameSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let id = self.id;
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.notify(Stop { id: id });
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();

        match &self.game_mode {
            GameMode::OneVsOne(one_vs_one_server) => {
                let msg = game::OneVsOneConnect {
                    addr: ctx.address(),
                    socket: addr.recipient(),
                    uid: self.id,
                    opponent: self.room_id.unwrap(),
                };
                one_vs_one_server
                    .send(msg)
                    .into_actor(self)
                    .then(|_res, _, ctx| {
                        match _res {
                            Ok(_) => {}
                            _ => ctx.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
            GameMode::Matchmaking(matchmaking_server) => {
                let msg = game::Connect {
                    addr: ctx.address(),
                    socket: addr.recipient(),
                    id: self.id,
                };
                matchmaking_server
                    .send(msg)
                    .into_actor(self)
                    .then(|_res, _, ctx| {
                        match _res {
                            Ok(_) => {}
                            _ => ctx.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
            GameMode::Tournament(tournament_server) => {
                let tournament_connect = game::TournamentConnect {
                    socket: addr.recipient(),
                    uid: self.id,
                    addr: ctx.address(),
                    tournament_id: self.room_id.unwrap(),
                };
                tournament_server
                    .send(tournament_connect)
                    .into_actor(self)
                    .then(|_res, _, ctx| {
                        match _res {
                            Ok(_) => {}
                            _ => ctx.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
        };
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        let msg = game::Disconnect { id: self.id };

        match &self.game_mode {
            GameMode::OneVsOne(game_server) => {
                game_server.do_send(msg);
            }
            GameMode::Matchmaking(matchmaking_server) => {
                matchmaking_server.do_send(msg);
            }
            GameMode::Tournament(tournament_server) => {
                tournament_server.do_send(msg);
            }
        }

        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => {}
            Ok(ws::Message::Text(s)) => {
                let msg = game::ClientMessage {
                    id: self.id,
                    msg: s.to_string(),
                };

                match &self.game_mode {
                    GameMode::OneVsOne(one_vs_one_server) => {
                        one_vs_one_server.do_send(msg);
                    }
                    GameMode::Matchmaking(matchmaking_server) => {
                        matchmaking_server.do_send(msg);
                    }
                    GameMode::Tournament(tournament_server) => {
                        tournament_server.do_send(msg);
                    }
                }
            }
            Err(e) => {
                println!("{}: an error occured in the game: {}", self.id, e);
                ctx.stop();
            }
        }
    }
}

impl Handler<Stop> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: Stop, ctx: &mut Self::Context) {
        println!("GameServer: Websocket CLient hearbeat failed, disconnecting!");

        let msg = game::Disconnect { id: msg.id };

        match &self.game_mode {
            GameMode::OneVsOne(game_server) => {
                game_server.do_send(msg);
            }
            GameMode::Matchmaking(matchmaking_server) => {
                matchmaking_server.do_send(msg);
            }
            GameMode::Tournament(tournament_server) => {
                tournament_server.do_send(msg);
            }
        }

        ctx.stop();
    }
}

impl Handler<game::Message> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: game::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

//  -------------------------- GAME ENDPOINTS ----------------------------
#[get("/game/matchmake")]
async fn matchmaking(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<MatchmakingServer>>,
) -> Result<HttpResponse, ApiError> {
    println!("HELLO");
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
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
    server: web::Data<Addr<TournamentServer>>,
    size: web::Path<u8>,
) -> Result<HttpResponse, ApiError> {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
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
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<TournamentServer>>,
    room_id: web::Path<UserId>,
) -> Result<HttpResponse, ApiError> {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

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
    stream: web::Payload,
    server: web::Data<Addr<OneVsOneServer>>,
    opponent_uid: web::Path<UserId>,
) -> Result<HttpResponse, ApiError> {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

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
