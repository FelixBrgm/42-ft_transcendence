
use super::error::ApiError;
use crate::http::db::Database;
use crate::http::db::models::NewUser;

use actix_web::get;
use actix_session::Session;
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse, HttpRequest, HttpMessage, http};
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use oauth2::basic::BasicClient;
use serde::Deserialize;
use serde_json;
use reqwest;

// ************************************************************ \\
//							  LOGIN
// ************************************************************ \\

/// Initiates the OAuth2 login process.
///
/// # Endpoint
/// `GET /auth/login`
///
/// # Description
/// - Initiates the login process using OAuth2 authorization code flow.
/// - Redirects the user to the intra server for authentication.
///
/// # Response
/// - If the user is already logged in, they will be redirected to the frontend.
/// - Otherwise, the user will be redirected to the authorization server for authentication.
///
#[get("/auth/login")]
async fn login(
	id: Option<Identity>,
	client: web::Data<BasicClient>,
	session: Session
	)
	-> Result<HttpResponse, ApiError>
	{

	// If user is already logged in redirect to frontend
	if id.is_some() {
		println!("(login) {:?} is already logged in", id.unwrap().id());
		return Ok(HttpResponse::Found().insert_header((LOCATION, "/")).finish());
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
	.append_header((http::header::LOCATION, auth_url.to_string()))
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

/// Handles the OAuth2 callback after successful authentication.
///
/// # Endpoint
/// `GET /auth/callback`
///
/// # Description
/// - Handles the callback from the intra server after successful authentication.
/// - Retrieves the authorization code, exchanges it for an access token, and retrieves user information.
/// - Logs in the user, updates session data, and interacts with the database.
///
/// # Query Parameters
/// - `code`: Authorization code received from the intra server.
/// - `state`: CSRF protection token received from the intra server.
///
/// # Response
/// - If the user is already logged in, they will be redirected to the frontend.
/// - After successful authentication, the user will be logged in, and their information will be added/updated in the database.
/// - They will then be redirected to the frontend.
///
#[get("/auth/callback")]
async fn callback(
	id: Option<Identity>,
	req: HttpRequest,
    session: Session,
	query: web::Query<AuthRequest>,
    client: web::Data<BasicClient>,
	database: web::Data<Database>
	)
	-> Result<HttpResponse, ApiError>
{
	// If user is already logged in redirect to frontend
	if id.is_some() {
		println!("(callback) {:?} is already logged in",id.unwrap().id());
		return Ok(HttpResponse::Found().insert_header((LOCATION, "/")).finish());
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
                "Failed to exchange token with 42 Intra: {}", e
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
	
	Ok(HttpResponse::Found()
   .insert_header((LOCATION, "/"))
   .finish())
}

fn extract_code_and_state(query: &web::Query<AuthRequest>)
	-> Result<(oauth2::AuthorizationCode, oauth2::CsrfToken), ApiError>
{
	// Check if authentication failed
	if query.code.is_none() || query.state.is_none() {
		return Err(ApiError::Unauthorized);
	}

	// Extract the code and state from the query parameters
	let code = oauth2::AuthorizationCode::new(query.code.clone().unwrap());
	let state = oauth2::CsrfToken::new(query.state.clone().unwrap());

	Ok((code, state))
}

async fn get_user_info(token: &str)
	-> Result<(i32, String, String), ApiError>
{
	let client = reqwest::Client::new();
    let user_info_endpoint = "https://api.intra.42.fr/v2/me";

    // Make the GET request with the access token in the Authorization header
    let Ok(response) = client
        .get(user_info_endpoint)
		.bearer_auth(token)
        .send()
        .await else {
			return Err(ApiError::InternalServerError);
	};

	let Ok(user_info) =  response.json::<serde_json::Value>().await else {
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

async fn interact_with_db(user_info: (i32, String, String), database:web::Data<Database>)
	-> Result<(), ApiError>
{
	let (id, login_d, avatar) = user_info;

	// todo: implement password
	match database.get_user_by_id(id)
	{
		Ok(user) => {
						database.update_user_status(id, "online")?;
						println!(" this user was found : {:?}", user);
					}
		Err(_) => {
			println!("adding user {}, {}",id, login_d);
			database.add_user(&NewUser{id, login: login_d, avatar})?;
		}
	}
	Ok(())
}

// ************************************************************ \\
//							  LOGOUT
// ************************************************************ \\

/// Logs out the authenticated user.
///
/// # Endpoint
/// `GET /auth/logout`
///
/// # Description
/// - Logs out the authenticated user.
/// - Updates user status in the database to "offline".
/// - Clears the user session.
///
/// # Response
/// - Redirects the user to the frontend after successful logout.
///
#[get("/auth/logout")]
async fn logout(
	id: Identity,
	database: web::Data<Database>
	)
	-> Result<HttpResponse, ApiError>
{
	println!("logging out user");

	database.update_user_status(id.id()?.parse()?, "offline")?;
	id.logout();

	Ok(HttpResponse::Found()
   .insert_header((LOCATION, "/"))
   .finish())
}