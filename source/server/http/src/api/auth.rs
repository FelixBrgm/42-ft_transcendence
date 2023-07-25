use super::errors::ApiError;
use crate::db::wrapper::Database;

use actix_web::http::header::LOCATION;
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, CsrfToken, Scope, PkceCodeChallenge, PkceCodeVerifier};
use actix_web::{web, HttpResponse, HttpRequest, Responder, http, get};
use actix_identity::Identity;
use actix_session::Session;
use serde::Deserialize;

pub fn init(cfg: &mut web::ServiceConfig)
{
	cfg.service(
		web::scope("/auth")
		 .route("/login", web::get().to(login))
		 .route("/callback", web::get().to(callback))
		 //  .route("/logout", web::get().to(logout))
	);
}

// Login route: Initiates the OAuth2 flow by redirecting the user to the authorization endpoint
async fn login(
	client: web::Data<BasicClient>,
	session: Session)
	-> Result<HttpResponse, ApiError> {

	// If user is already logged in redirect to frontend

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

#[derive(Debug, Deserialize)]
pub struct AuthRequest{
	code: Option<String>,
	state: Option<String>,
	error: Option<String>,
    error_description: Option<String>,
}

// Your application (callback URL) receives the authorization code in the query parameter of the redirect URI.
// Your application then exchanges this authorization code for an access token by making a secure, server-to-server request to the OAuth provider's token endpoint.
// Along with the authorization code, you'll also need to provide the client ID, client secret, redirect URI, and the grant_type=authorization_code.
async fn callback(
	req: HttpRequest,
    client: web::Data<BasicClient>,
	query: web::Query<AuthRequest>,
    session: Session)
	-> Result<HttpResponse, ApiError>
{

	// If user is already logged in redirect to frontend

	// Check if authentication failed
	if let Some(err) = &query.error {
        let reason = query
			.error_description
            .as_ref()
            .map_or(err.clone(), |desc| format!("{}: {}", err, desc));

        return Ok(HttpResponse::Unauthorized().body(reason));
    }

	if query.code.is_none() || query.state.is_none() {
		return Ok(HttpResponse::InternalServerError().body("Unexpected callback state."));
	}

	// Extract the code and state from the query parameters
	let code = oauth2::AuthorizationCode::new(query.code.clone().unwrap());
	let state = oauth2::CsrfToken::new(query.state.clone().unwrap());

	 // Verify the state for CSRF protection
	 let session_state = session.get::<String>("state")?;
	 if session_state.is_none() {
		 return Err(ApiError::BadRequest("No state".to_string()));
	 }
	 if session_state.unwrap() != *state.secret() {
		 return Err(ApiError::BadRequest("Invalid state".to_string()));
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
			log::error!("Failed to exchange token with 42 Intra: {}", e);
            return Err(ApiError::BadRequest(format!(
                "Failed to exchange token with 42 Intra: {}",
                e
            )));
        }
    };

	// Remove old session data
    // And save the token from 42 Intra in the session.
    session.remove("pkce_verifier");
    session.remove("state");
    session.insert("token", token)?;

	// let user_info_response = get_user_info(&token. ).await;

    // // Handle the case where neither error nor code is present (unexpected state)		
	Ok(HttpResponse::Found()
   .insert_header((LOCATION, "/"))
   .finish())

}


// async fn get_user_info(token: &str) -> Result<reqwest::Response, reqwest::Error> {
//     let client = reqwest::Client::new();
//     let url = "https://api.intra.42.fr/v2/me";

//     // Make the GET request with the access token in the Authorization header
//     let response = client
//         .get(url)
//         .header("Authorization", format!("Bearer {}", token))
//         .send()
//         .await?;

//     Ok(response)
// }