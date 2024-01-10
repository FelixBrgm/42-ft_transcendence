use super::error::ApiError;
use actix::prelude::*;
// use actix_identity::Identity;

use crate::game;
use crate::db::Database;
use actix::{Actor, Addr, StreamHandler};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::{debug, error};
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct GameSession {
	id: usize,
	addr: Addr<game::GameServer>,
	hb: Instant,
}

impl GameSession {
	pub fn new(id: usize, addr: Addr<game::GameServer>) -> GameSession {
		GameSession {
			id,
			addr,
			hb: Instant::now(),
		}
	}
}

impl GameSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("GameServer: Websocket CLient hearbeat failed, disconnecting!");

                act.addr.do_send(game::Disconnect {
                    id: act.id,
                });

                ctx.stop();

                return;
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
        self.addr
            .send(game::Connect {
                socket: addr.recipient(),
                id: self.id,
            })
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

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(game::Disconnect {
            id: self.id,
        });
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

			if let Some(c) = s.chars().last() {
				self.addr.do_send(game::ClientMessage {
					id: self.id,
					msg: c,
				})
			}
			else {
				println!("in the text thing something went wrong");
			}
			},
            Err(e) => {
                println!("{}: an error occured in the game: {}", self.id, e);
                ctx.stop();
            }
        }
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

#[get("/game")]
async fn server(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<game::GameServer>>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {

	let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    match ws::start(
        GameSession::new(client_id, server.get_ref().clone()),
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