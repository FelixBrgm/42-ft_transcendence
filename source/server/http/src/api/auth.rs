use crate::db::wrapper::Database;
use actix_web::{get, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_identity::Identity;
use anyhow::{Result};

pub fn init(cfg: &mut web::ServiceConfig)
{
	// cfg.service(
	// 	web::scope("/auth")
	// 	 .route("/login", web::get().to())
	// 	 .route("/logout", web::get().to())
	// 	 .route("/callback", web::get().to())
	// );
}
