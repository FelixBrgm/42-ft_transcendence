
use crate::db::wrapper::Database;
use actix_web::{get, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_identity::Identity;
use anyhow::{Result};

pub fn init(cfg: &mut web::ServiceConfig)
{
	cfg.service(
		web::scope("/client")
		 .route("", web::get().to(client))
	);
}

// Handler functinos should return
// a struct that implements the responder trait
// or Result with Ok(Responder trais) && Err(response error)

async fn client(identity: Option<Identity>, db: web::Data<Database>) -> impl Responder {
	
	match identity {
		Some(user) => {println!("user is {:?}", user.id())},
		None => println!("no identity attached"),
	};

	web::Json("hello world".to_string())
}

