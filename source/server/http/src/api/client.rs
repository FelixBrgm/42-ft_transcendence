
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

async fn client( db: web::Data<Database>) -> web::Json<String> {
	
	web::Json("hello world".to_string())
	// match db.get_client_name("herbert"){
	// 	Ok(client) => {println!("the get client is: {:?}", client)},
	// 	Err(_) => {println!("couldn't retrieve client")},
	// }
	// HttpResponse::Ok()
}

