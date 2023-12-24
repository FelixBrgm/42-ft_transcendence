
use actix_identity::Identity;
use actix_web_actors::ws::{Message, WebsocketContext};
use super::error::ApiError;

use actix::{Actor, StreamHandler};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

// url -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" -H "Sec-WebSocket-Version: 13" -H "Sec-WebSocket-Key: $(head -c 16 /dev/urandom | base64)" http://localhost:8080/ws

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		match msg {
            Ok(Message::Ping(ping)) => ctx.pong(&ping),
            Ok(Message::Text(text)) => {
                println!("Received message: {}", text);
                ctx.text(format!("Echo: {}", text));
            }
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("/ws")]
async fn server(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, ApiError> {
	let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
   Ok(HttpResponse::Ok().body("test"))
}
