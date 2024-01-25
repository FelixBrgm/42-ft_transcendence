use actix::prelude::*;



use actix::{Actor, Addr, StreamHandler};

use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

use crate::chat::server::*;
pub struct WsActor {
    id: usize,
    hb: Instant,
    addr: Addr<ChatServer>,
}

impl WsActor {
    pub fn new(id: usize, addr: Addr<ChatServer>) -> WsActor {
        WsActor {
            id: id,
            addr: addr,
            hb: Instant::now(),
        }
    }
}

impl WsActor {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("ChatServer: Websocket CLient hearbeat failed, disconnecting!");

                act.addr.do_send(Disconnect {
                    id: act.id,
                });

                ctx.stop();

                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
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
        self.addr.do_send(Disconnect {
            id: self.id,
        });
        Running::Stop
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
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
            Ok(ws::Message::Text(s)) => self.addr.do_send(ClientMessage {
                id: self.id,
                msg: s.to_string(),
            }),
            Err(e) => {
                println!("{}: an error occured in the chat: {}", self.id, e);
                ctx.stop();
            }
        }
    }
}

impl Handler<ChatMessage> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
