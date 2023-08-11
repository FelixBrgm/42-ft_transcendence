use super::error::ApiError;
use crate::http::db::Database;

use actix_web::{get, post, web, HttpResponse};
use actix_identity::Identity;
use anyhow::Result;

#[get("/users")]
async fn users_get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError>
{
	let users = db.get_users()?;
    Ok(HttpResponse::Ok().json(&users))
}