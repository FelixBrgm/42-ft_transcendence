use super::errors::ApiError;
use crate::db::wrapper::Database;
use crate::db::models;

use actix_web::http::header::LOCATION;
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::{AuthorizationCode, CsrfToken, Scope, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use actix_web::{web, HttpResponse, HttpRequest, HttpMessage, Responder, http, get};
use actix_identity::Identity;
use actix_session::Session;
use openssl::pkey::Id;
use serde::Deserialize;
use serde_json;
use reqwest;

// add to db			DONE
// logout				
// add the log
// testing
// make it pretty
// set default avatar

pub fn init(cfg: &mut web::ServiceConfig)
{
	cfg.service(
		web::scope("/auth")
			.route("/login", web::get().to(login))
			.route("/callback", web::get().to(callback))
			.route("/logout", web::get().to(logout))
	);
}

// ************************************************************ \\
//							  LOGIN
// ************************************************************ \\

// Login route: Initiates the OAuth2 flow by redirecting the user to the authorization endpoint
async fn login(
	id: Option<Identity>,
	client: web::Data<BasicClient>,
	session: Session)
	-> Result<HttpResponse, ApiError> {

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
pub struct AuthRequest{
	code: Option<String>,
	state: Option<String>,
	error: Option<String>,
    error_description: Option<String>,
}

async fn callback(
	id: Option<Identity>,
	req: HttpRequest,
    client: web::Data<BasicClient>,
	query: web::Query<AuthRequest>,
    session: Session,
	database: web::Data<Database>
)
	-> Result<HttpResponse, ApiError>
{
	// If user is already logged in redirect to frontend
	if id.is_some() {
		println!("(callback) {:?} is already logged in",id.unwrap().id());
		return Ok(HttpResponse::Found().insert_header((LOCATION, "/")).finish());
	}

	// Check if authentication failed
	if let Some(err) = &query.error {
        let reason = query
			.error_description
            .as_ref()
            .map_or(err.clone(), |desc| format!("{}: {}", err, desc));

        return Ok(HttpResponse::Unauthorized().body(reason));
    }

	if query.code.is_none() || query.state.is_none() {
		return Err(ApiError::InternalServerError);
	}

	// Extract the code and state from the query parameters
	let code = oauth2::AuthorizationCode::new(query.code.clone().unwrap());
	let state = oauth2::CsrfToken::new(query.state.clone().unwrap());

	 // Verify the state for CSRF protection
	 let session_state = session.get::<String>("state")?;
	 if session_state.is_none() {
		 return Err(ApiError::BadRequest("No state (CSRF)".to_string()));
	 }
	 if session_state.unwrap() != *state.secret() {
		 return Err(ApiError::BadRequest("Invalid state (CSRF)".to_string()));
	 }

	// Retrieve the pkceVerifier from the session
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

	// Retrieve the user information
	let user_info = get_user_info(token.access_token().secret()).await?;

	Identity::login(&req.extensions(), (user_info.0).to_string())?;

	// add to database if not already added
	interact_with_db(user_info, database).await?;
	
	Ok(HttpResponse::Found()
   .insert_header((LOCATION, "/"))
   .finish())

}

async fn get_user_info(token: &str) -> Result<(i32, String, String), ApiError>
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

async fn interact_with_db(user_info: (i32, String, String), database:web::Data<Database>) -> Result<(), ApiError>
{
	let (id, login, avatar) = user_info;

	// todo: implement password
	match database.get_user_by_id(id)
	{
		Ok(user) => { println!(" this user was found : {:?}", user);}
		Err(_) => {
			println!("adding user {}, {}",id, login);
			database.add_user(&models::NewUser{id, login, avatar})?;
		}
	}
	Ok(())
}

// ************************************************************ \\
//							  LOGOUT
// ************************************************************ \\

async fn logout(
	id: Option<Identity>,
	database: web::Data<Database>
) -> Result<HttpResponse, ApiError>
{
	if let Some(id) = id{
		println!("logging out user");
		database.update_user(&models::UpdateUser{
			id: id.id()?.parse()?,
			status: Some("offline".to_string()),
			..Default::default()
		})?;

		id.logout();
	}
	
	println!("redirecting user");

	Ok(HttpResponse::Found()
   .insert_header((LOCATION, "/"))
   .finish())
}
