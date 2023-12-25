
use actix_identity::Identity;
use actix_web_actors::ws::{Message, WebsocketContext};
use actix::prelude::*;
use super::error::ApiError;

use crate::chat::{ChatServer, ClientMessage};
use actix::{Actor, StreamHandler, Addr};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

struct ChatSession {
	id: usize,
	addr: Addr<ChatServer>,
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

	fn started(&mut self, context: &mut Self::Context) {

		// start heartbeat if i want to implement this

		// let addr = context.address();
		// self.addr.send(<ChatServer as Handler>::Connect {
		// 	addr: addr.recipient(),
		// })
		// .into_actor(self)
		// .then(|res, act, ctx| {
		// 	match res {
		// 		Ok(res) => act.id = res,
		// 		// something is wrong
		// 		_ => context.stop(),
		// 	}
		// 	fut::ready(())
		// })
		// .wait(context);
	}

	// stopping
}

impl Handler<ClientMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
		println!("chatsession message handler");
        // ctx.text(msg.0);
    }
}


/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		println!("streamhandler of chatsession not implememted yet");
	}
}


#[get("/ws")]
async fn server(req: HttpRequest, stream: web::Payload, server: web::Data<Addr<ChatServer>>) -> Result<HttpResponse, ApiError> {
	let resp = ws::start(ChatSession{
		id: 0,
		addr: server.get_ref().clone(),
	}, &req, stream);
    println!("{:?}", resp);
   Ok(HttpResponse::Ok().body("test"))
}
