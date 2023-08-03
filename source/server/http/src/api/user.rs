
use crate::db::wrapper::Database;
use crate::api::errors::ApiError;
use actix_web::{get, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_identity::Identity;
use anyhow::{Result};

pub fn init(cfg: &mut web::ServiceConfig)
{
	cfg.service(
		web::scope("/user")
		 .route("", web::get().to(get_user))
		//  .route("", web::post().to());
	);
}

async fn get_user(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
	println!("GET /api/user");
	if let Ok(id_str) = identity.id()
	{
		println!("id is {}", id_str);
		if let Ok(id) = id_str.parse::<i32>() {
            let user = db.get_user_by_id(id);
			if let Ok(user) = user{
				println!("user is {:?}", user);
				return Ok(HttpResponse::Ok().json(&user));
			}
        }
		return Err(ApiError::InternalServerError);
	}
	return Err(ApiError::Unauthorized)
}

