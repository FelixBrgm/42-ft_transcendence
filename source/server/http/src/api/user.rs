
use crate::db::wrapper::Database;
use crate::api::errors::ApiError;
use actix_web::{get, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_identity::Identity;
use anyhow::{Result};

pub fn init(cfg: &mut web::ServiceConfig)
{
	cfg.service(
		web::scope("/user")
		 .route("", web::get().to(user_get))
		 .route("", web::put().to(user_put))
		 
	);
}

// returns the information of the user sending the request
async fn user_get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {

	let Ok(id) = identity.id() else {
		return Err(ApiError::Unauthorized)
	};

	let user = db.get_user_by_id(id.parse::<i32>()?)?;
	Ok(HttpResponse::Ok().json(&user))
}

async fn user_put(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
	
	let Ok(id) = identity.id() else {
		return Err(ApiError::Unauthorized)
	};

	// let result = db.set_user();

    // match result {
    //     Ok(_) => Ok(HttpResponse::Ok().finish()),
    //     Err(_) => Err(ApiError::InternalServerError),
    // }

	Ok(HttpResponse::Ok().finish())
}

// // updates the information of the user sending the request
// async fn user_put(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
// 	println!("PUT /api/user");
// 	if let Ok(id_str) = identity.id()
// 	{
// 		if let Ok(id) = id_str.parse::<i32>() {
//             let user = db.get_user_by_id(id);
// 			if let Ok(user) = user{
// 				println!("user is {:?}", user);
// 				return Ok(HttpResponse::Ok().json(&user));
// 			}
//         }
// 		return Err(ApiError::InternalServerError);
// 	}
// 	return Err(ApiError::Unauthorized)
// }

