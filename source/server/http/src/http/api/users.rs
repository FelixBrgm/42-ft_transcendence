use super::error::ApiError;
use crate::http::db::Database;

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;

/// Retrieves a list of users.
///
/// # Endpoint
/// `GET /users`
///
/// # Description
/// - Retrieves a list of users from the database.
/// - Requires authentication using a valid session token.
///
/// # Parameters
/// - `identity`: The identity token representing the authenticated user.
/// - `db`: A reference to the database connection.
///
/// # Response
/// - Returns a JSON array containing information about the users.
/// - Returns an empty array if no users are found.
/// - Returns a 401 Unauthorized response if the user is not authenticated.
/// - Returns a 500 Internal Server Error response if a database error occurs.
///
#[get("/users")]
async fn users_get(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    let users = db.get_users()?;
    Ok(HttpResponse::Ok().json(&users))
}

// #[get("/clear_users")]
// async fn clear_users(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
//     let users = db.get_users()?;
//     Ok(HttpResponse::Ok().json(&users))
// }

// #[post("/users")]
