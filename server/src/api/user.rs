use super::error::ApiError;
use crate::db::{models::UpdateUser, Database};

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;
use serde::Serialize;
use crate::db::models::User;


#[derive(Debug, Serialize)]
pub struct ProtectedUser {
    pub id: i32,
    pub intra: String,
    pub alias: String,
    pub avatar: String,
    pub status: String,
    pub wins: i32,
    pub losses: i32,
}

impl ProtectedUser {
    pub fn new(user :&User) -> Self {
        ProtectedUser {
            id: user.id,
            intra: user.intra.to_string(),
            alias: user.alias.to_string(),
            avatar: user.avatar.to_string(),
            status: user.status.to_string(),
            wins: user.wins,
            losses: user.losses,
        }
    }
}

#[get("/api/user")]
async fn get(identity: Identity, db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let id = identity.id()?;

    match db.get_user_by_id(id.parse::<i32>()?)? {
        Some(user) => Ok(HttpResponse::Ok().json(&user)),
        None => Err(ApiError::NotFound),
    }
}

#[post("/api/user")]
async fn post(
    identity: Identity,
    update_user: web::Json<UpdateUser>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user = update_user.into_inner();
    let uid = identity.id()?.parse::<i32>()?;

    let msg = format!("User {} updated succesfully!", uid);
    match db.update_user(&user, uid) {
        Ok(_) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[get("/api/users")]
async fn list(
	_: Identity,
	db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {

	match db.get_all_users() {
		Ok(users) => {
            let protected_users: Vec<ProtectedUser> = users
            .iter()
            .map(|user| ProtectedUser::new(user)) 
            .collect(); 

            Ok(HttpResponse::Ok().json(protected_users))
        },
		Err(_) => Err(ApiError::InternalServerError),
	}
}

#[get("/api/user/{user_id}")]
async fn find(
    _: Identity,
    user_id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user = user_id.into_inner();

    match db.get_user_by_id(user) {
        Ok(user) => { 
            let protected_user = user.map(|u| ProtectedUser::new(&u));
            Ok(HttpResponse::Ok().json(protected_user))
        },
        Err(_) => Err(ApiError::InternalServerError),
    }
}


#[get("/api/user/check/{user_id}")]
async fn check(
    _: Identity,
    user_id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user = user_id.into_inner();

    match db.get_user_by_id(user) {
        Ok(_) => { 
            Ok(HttpResponse::Ok().json(true))
        },
        Err(_) => Ok(HttpResponse::Ok().json(false)),
    }
}
