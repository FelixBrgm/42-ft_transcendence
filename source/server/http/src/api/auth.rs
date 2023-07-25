use crate::db::wrapper::Database;
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, CsrfToken, Scope, PkceCodeChallenge};
use actix_web::{web, HttpResponse, HttpRequest, Responder, http, get};
use actix_identity::Identity;
use actix_session::Session;
use serde::Deserialize;
use super::errors::ApiError;

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
async fn login(client: web::Data<BasicClient>, session: Session) -> Result<HttpResponse, ApiError> {

	// If user is already logged in redirect to frontend

	// proof key for code exchange
	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
	
	// Create the authorization URL and redirect the user to it
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
    session: Session,) -> Result<HttpResponse, ApiError>
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

	// // Token exchange using authorization code
	// if let Some(code) = &query.code {
    //     let code = AuthorizationCode::new(code.to_string());

    //     match client.exchange_code(code).request_async(async_http_client).await {
    //         Ok(token) => {
    //             // Use the token to make API requests on behalf of the user
    //             // For example, you can store the token and use it later to access the 42API
    //             // Here, we are just displaying a success message with the token value
    //             return HttpResponse::Ok().body(format!("Token: {}", token.access_token().secret()));
    //         }
    //         Err(e) => {
    //             return HttpResponse::InternalServerError().body(format!("Token exchange failed: {}", e));
    //         }
    //     }
    // }

    // // Handle the case where neither error nor code is present (unexpected state)
   Ok(HttpResponse::Ok().body(" Successfull callback!"))

}
