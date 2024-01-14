use super::error::ApiError;
use actix::prelude::*;
// use actix_identity::Identity;

use crate::db::Database;
use crate::game;
use crate::game::matchmake::MatchmakingServer;
use crate::game::tournament::TournamentServer;
use crate::game::one_vs_one::OneVsOneServer;

use actix::{Actor, Addr, StreamHandler};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::{debug, error};
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone)]
enum GameMode {
    OneVsOne(Addr<OneVsOneServer>),
    Matchmaking(Addr<MatchmakingServer>),
    Tournament(Addr<TournamentServer>),
}

struct GameSession {
    id: usize,
    game_mode: GameMode,
    hb: Instant,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Stop{
	pub id: usize
}

impl GameSession {
    fn new_one_vs_one(id: usize, one_vs_one_server: Addr<OneVsOneServer>) -> Self {
        GameSession {
            id,
            game_mode: GameMode::OneVsOne(one_vs_one_server),
            hb: Instant::now(),
        }
    }

    fn new_matchmaking(id: usize, matchmaking_server: Addr<MatchmakingServer>) -> Self {
        GameSession {
            id,
            game_mode: GameMode::Matchmaking(matchmaking_server),
            hb: Instant::now(),
        }
    }

    fn new_tournament(id: usize, tournament_server: Addr<TournamentServer>) -> Self {
        GameSession {
            id,
            game_mode: GameMode::Tournament(tournament_server),
            hb: Instant::now(),
        }
    }
}

impl GameSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
		let id = self.id;
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
				let addr = ctx.address().do_send(Stop{id: id});
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
		let msg = game::Connect{
			socket: addr.recipient(),
            id: self.id,
		};

		match &self.game_mode {
			GameMode::OneVsOne(one_vs_one_server) => {
				one_vs_one_server.send(msg)
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
				matchmaking_server.send(msg)
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
				tournament_server.send(msg)
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
			},
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
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
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

#[get("/game/tournament")]
async fn tournament(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<TournamentServer>>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    match ws::start(
        GameSession::new_tournament(client_id, server.get_ref().clone()),
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

#[get("/game/one_vs_one")]
async fn one_vs_one(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<OneVsOneServer>>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    match ws::start(
        GameSession::new_one_vs_one(client_id, server.get_ref().clone()),
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