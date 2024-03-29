use super::error::ApiError;
use crate::db::models::NewUser;
use crate::db::Database;

use actix_identity::Identity;
use actix_session::Session;
use actix_web::get;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use oauth2::basic::BasicClient;
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use serde::Deserialize;

use reqwest;
use serde_json;

// #[get("/api/auth/fake/{uid}")]
// async fn fake(
//     id: Option<Identity>,
//     uid: web::Path<i32>,
//     req: HttpRequest,
//     db: web::Data<Database>,
// ) -> Result<HttpResponse, ApiError> {
//     if id.is_some() {
//         println!("(login) {:?} is already logged in", id.unwrap().id());
//         return Ok(HttpResponse::Found().finish());
//     }

//     let uid = uid.into_inner();
//     db.add_user(&NewUser {
//         id: uid,
//         intra: format!("user {}", uid),
//         alias: format!("."),
//         avatar: format!("."),
//     })?;

//     Identity::login(&req.extensions(), uid.to_string())?;
//     Ok(HttpResponse::Ok().finish())
// }

#[get("/api/auth/login")]
async fn login(
    id: Option<Identity>,
    client: web::Data<BasicClient>,
    database: web::Data<Database>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    // If user is already logged in redirect to frontend
    if id.is_some() {
        let id = id.unwrap().id()?.parse()?;
        println!("(login) {:?} is already logged in", id);

        database.update_user_status(id, "online")?;

        let inend_url = std::env::var("INEND_URL").expect("INEND_URL must be set");
        return Ok(HttpResponse::Found()
            .insert_header((LOCATION, inend_url))
            .finish());
    }

    // proof key for code exchange
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Create the authorization URL
    let (auth_url, csrf_token) = &client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .url();

    // Store pkce_verifier and state in session for CSRF protection
    session.insert("pkce_verifier", pkce_verifier)?;
    session.insert("state", csrf_token.secret().clone())?;

    // Redirect the user to the authorization URL
    Ok(HttpResponse::Found()
        .append_header((LOCATION, auth_url.to_string()))
        .finish())
}

// ************************************************************ \\
//							 CALLBACK
// ************************************************************ \\

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: Option<String>,
    state: Option<String>,
}

#[get("/api/auth/callback")]
async fn callback(
    id: Option<Identity>,
    req: HttpRequest,
    session: Session,
    query: web::Query<AuthRequest>,
    client: web::Data<BasicClient>,
    database: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    // If user is already logged in redirect to frontend
    if id.is_some() {
        let id = id.unwrap().id()?.parse()?;
        println!("(callback) {:?} is already logged in", id);

        database.update_user_status(id, "online")?;

        let inend_url = std::env::var("INEND_URL").expect("INEND_URL must be set");
        return Ok(HttpResponse::Found()
            .insert_header((LOCATION, inend_url))
            .finish());
    }

    let (code, state) = extract_code_and_state(&query)?;

    // Verify the state for CSRF protection
    let session_state = session.get::<String>("state")?;
    if session_state.is_none() {
        return Err(ApiError::BadRequest("No state (CSRF)".to_string()));
    }
    if session_state.unwrap() != *state.secret() {
        return Err(ApiError::BadRequest("Invalid state (CSRF)".to_string()));
    }

    let Some(pkce_verifier) = session.get::<PkceCodeVerifier>("pkce_verifier")? else {
        return Err(ApiError::InternalServerError);
    };

    let token = &client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    let token = match token {
        Ok(token) => token,
        Err(e) => {
            return Err(ApiError::BadRequest(format!(
                "Failed to exchange token with 42 Intra: {}",
                e
            )));
        }
    };

    // Update session data
    session.remove("pkce_verifier");
    session.remove("state");
    session.insert("token", token)?;

    let user_info = get_user_info(token.access_token().secret()).await?;

    Identity::login(&req.extensions(), (user_info.0).to_string())?;

    interact_with_db(user_info, database).await?;

    // add the user to the socket hashmap
    let inend_url = std::env::var("INEND_URL").expect("INEND_URL must be set");
    return Ok(HttpResponse::Found()
        .insert_header((LOCATION, inend_url))
        .finish());
}

fn extract_code_and_state(
    query: &web::Query<AuthRequest>,
) -> Result<(oauth2::AuthorizationCode, oauth2::CsrfToken), ApiError> {
    // Check if authentication failed
    if query.code.is_none() || query.state.is_none() {
        return Err(ApiError::Unauthorized);
    }

    // Extract the code and state from the query parameters
    let code = oauth2::AuthorizationCode::new(query.code.clone().unwrap());
    let state = oauth2::CsrfToken::new(query.state.clone().unwrap());

    Ok((code, state))
}

async fn get_user_info(token: &str) -> Result<(i32, String, String), ApiError> {
    let client = reqwest::Client::new();
    let user_info_endpoint = "https://api.intra.42.fr/v2/me";

    // Make the GET request with the access token in the Authorization header
    let Ok(response) = client
        .get(user_info_endpoint)
        .bearer_auth(token)
        .send()
        .await
    else {
        return Err(ApiError::InternalServerError);
    };

    let Ok(user_info) = response.json::<serde_json::Value>().await else {
        return Err(ApiError::InternalServerError);
    };

    // Extract `id`, `login`, and `avatar` from the `user_info` Value
    let intra_id = user_info["id"]
        .as_i64()
        .ok_or(ApiError::InternalServerError)? as i32;

    let intra_login = user_info["login"]
        .as_str()
        .ok_or(ApiError::InternalServerError)?
        .to_string();

    // todo: put this in .env and define a default avatar
    let intra_avatar = user_info["image"]["versions"]["medium"]
        .as_str()
        .unwrap_or("https://i.pinimg.com/564x/bc/5d/17/bc5d173a3001839b5f4ec29efad072ae.jpg")
        .to_string();

    Ok((intra_id, intra_login, intra_avatar))
}

use hex;
use rand::thread_rng;
use rand::RngCore;

async fn interact_with_db(
    user_info: (i32, String, String),
    database: web::Data<Database>,
) -> Result<(), ApiError> {
    let (id, login_d, avatar) = user_info;

    let mut rng = thread_rng();
    let mut token_bytes = [0u8; 16];
    rng.fill_bytes(&mut token_bytes);

    let token = hex::encode(token_bytes);

    match database.get_user_by_id(id) {
        Ok(user) => {
            database.update_user_status(id, "online")?;
            println!(" this user was found : {:?}", user);
        }
        Err(_) => {
            println!("adding user {}, {}", id, login_d);
            database.add_user(&NewUser {
                id,
                intra: login_d.to_string(),
                alias: login_d,
                avatar,
                token,
            })?;
        }
    }
    Ok(())
}

// ************************************************************ \\
//							  LOGOUT
// ************************************************************ \\

#[get("/api/auth/logout")]
async fn logout(id: Identity, database: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    database.update_user_status(id.id()?.parse()?, "offline")?;
    id.logout();
    Ok(HttpResponse::Ok().json("User logged out!"))
}

// ************************************************************ \\
//							 CHECK
// ************************************************************ \\

#[get("/api/auth/check")]
async fn check(id: Option<Identity>) -> Result<HttpResponse, ApiError> {
    match id {
        Some(_) => Ok(HttpResponse::Ok().json("User is logged in!")),
        None => Ok(HttpResponse::Ok().json("User isn't logged in!")),
    }
}
