use crate::db::wrapper::Database;
// use actix_web::{get, web, HttpRequest, HttpMessage, HttpResponse, Responder};
// use actix_identity::Identity;
// use anyhow::{Result};
use actix_web::{web, HttpResponse, HttpRequest, Responder, http, get};
use actix_identity::Identity;
use actix_session::Session;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, PkceCodeChallenge};

pub fn init(cfg: &mut web::ServiceConfig)
{
	cfg.service(
		web::scope("/auth")
		 .route("/login", web::get().to(login))
		//  .route("/logout", web::get().to(logout))
		 .route("/callback", web::get().to(callback))
	);
}

// Login route: Initiates the OAuth2 flow by redirecting the user to the authorization endpoint
async fn login(client: web::Data<BasicClient>, session: Session,) -> impl Responder {

	// proof key for code exchange
	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
	
	// Create the authorization URL and redirect the user to it
	let (auth_url, csrf_token) = &client
	.authorize_url(CsrfToken::new_random)
	.set_pkce_challenge(pkce_challenge)
	.url();

	// Store pkce_verifier and state in session for CSRF protection
	session.insert("pkce_verifier", pkce_verifier).expect("pkce insert failed");
	session.insert("state", csrf_token.secret().clone()).expect("csrf_state insert failed");

	// Redirect the user to the authorization URL
	HttpResponse::Found()
	.append_header((http::header::LOCATION, auth_url.to_string()))
	.finish()
}



// Your application (callback URL) receives the authorization code in the query parameter of the redirect URI.
// Your application then exchanges this authorization code for an access token by making a secure, server-to-server request to the OAuth provider's token endpoint.
// Along with the authorization code, you'll also need to provide the client ID, client secret, redirect URI, and the grant_type=authorization_code.
async fn callback() -> impl Responder
{
	HttpResponse::Ok()
	.body("Hello, this is the callback response!".to_string())
}