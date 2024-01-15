use super::error::ApiError;
use actix::prelude::*;
use actix_identity::Identity;


use crate::chat;
use crate::db::Database;
use actix::{Actor, Addr, StreamHandler};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct ChatSession {
    id: usize,
    room: usize,
    hb: Instant,
    addr: Addr<chat::ChatServer>,
}

impl ChatSession {
    pub fn new(id: usize, room: usize, addr: Addr<chat::ChatServer>) -> ChatSession {
        ChatSession {
            id: id,
            room: room,
            addr: addr,
            hb: Instant::now(),
        }
    }
}

impl ChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("ChatServer: Websocket CLient hearbeat failed, disconnecting!");

                act.addr.do_send(chat::Disconnect {
                    id: act.id,
                    room_id: act.room,
                });

                ctx.stop();

                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(chat::Connect {
                addr: addr.recipient(),
                id: self.id,
                room_id: self.room,
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
        self.addr.do_send(chat::Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
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
            Ok(ws::Message::Text(s)) => self.addr.do_send(chat::ClientMessage {
                id: self.id,
                msg: s.to_string(),
                room_id: self.room,
            }),
            Err(e) => {
                println!("{}: an error occured in the chat: {}", self.id, e);
                ctx.stop();
            }
        }
    }
}

impl Handler<chat::Message> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: chat::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

// TODO: need to load all the past messages in the room
#[get("/ws/{room_id}")]
async fn server(
	req: HttpRequest,
	// identity: Identity,
    stream: web::Payload,
    server: web::Data<Addr<chat::ChatServer>>,
    room_id: web::Path<usize>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let rid = room_id.into_inner();
	let uid = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
	// let uid = identity.id()?.parse::<usize>()?;

	if !db.check_room(rid as i32)? {
		return Err(ApiError::BadRequest("The Room is not found.".to_owned()));
	}

	if !db.check_connection(uid as i32, rid as i32)? {
		return Err(ApiError::BadRequest("The User didn't join the Room.".to_owned()));
	}

    match ws::start(
        ChatSession::new(uid, rid.try_into().unwrap(), server.get_ref().clone()),
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
