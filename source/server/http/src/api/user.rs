
use crate::db::wrapper::Database;
use crate::api::errors::ApiError;
use actix_web::{get, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_identity::Identity;
use anyhow::{Result};

pub fn init(cfg: &mut web::ServiceConfig)
{
	cfg.service(
		web::scope("/user")
		 .route("", web::get().to(user))
	);
}

// Handler functinos should return
// a struct that implements the responder trait
// or Result with Ok(Responder trais) && Err(response error)

async fn user(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
	println!("gets here");
	if let Ok(id_str) = identity.id()
	{
		println!("id is {}", id_str);
		if let Ok(id) = id_str.parse::<i32>() {
            // Now you can use id_i32 as an i32 value

			db.show_users();
            let user = db.get_user_by_id(id);
			if let Ok(user) = user{
				println!("user is {:?}", user);
				return Ok(HttpResponse::Ok().json(&user));
			}
			else {
				return Ok(HttpResponse::Ok().json("this is here"));
			}
        }
	}
	return Err(ApiError::Unauthorized)
}

