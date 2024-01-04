use super::error::ApiError;
use actix::prelude::*;
use actix_identity::Identity;
use actix_web_actors::ws::{Message, WebsocketContext};

use crate::chat;
use actix::{Actor, Addr, StreamHandler};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
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
                println!(" Websocket CLient hearbeat failes, disconnecting!");

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
        println!("websocket session started");

        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(chat::Connect {
                addr: addr.recipient(),
                self_id: self.id,
                room_id: self.room,
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {}
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
        println!("chatsession message handler");
        ctx.text(msg.0);
    }
}

#[get("/ws")]
async fn server(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<chat::ChatServer>>,
) -> Result<HttpResponse, ApiError> {
    println!("WebSocket connection requested for URL: {}", req.uri());
    let resp = ws::start(
        ChatSession::new(0, 0, server.get_ref().clone()),
        &req,
        stream,
    );

    println!("WebSocket response: {:?}", resp);

    match resp {
        Ok(ws) => Ok(ws),
        Err(err) => {
            eprintln!("Error during WebSocket handshake: {:?}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
