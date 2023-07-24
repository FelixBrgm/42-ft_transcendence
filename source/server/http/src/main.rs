// #[macro_use]
// extern crate diesel;

#![allow(dead_code)]
#![allow(unused_imports)]
#[allow(unused_variables)]

mod db;
mod api;

use db::wrapper::Database;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder};
use actix_web::middleware::Logger;
use actix_web::{http::header ,cookie::Key};
use actix_identity::IdentityMiddleware;
use actix_cors::Cors;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::url::Url;
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl, StandardTokenResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// std::env::set_var("RUST_LOG", "debug");
	// std::env::set_var("RUST_BACKTRACE", "1");
	// env_logger::init();

	let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
	let db = Database::new(&database_url);
    println!("Database connection established!");

	let auth_client = setup_oauth_client();
	println!("Authentication established!");
	
    // Start the Actix Web server
	HttpServer::new( move || {
		
			// cookie::key
			
			let cors = Cors::default()
			.allow_any_origin()
			.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
			.allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
			.supports_credentials();
	
			App::new()
			.app_data(web::Data::new(db.clone()))
			.app_data(web::Data::new(auth_client.clone()))
			.wrap(cors)
			.wrap(Logger::default())
			.wrap(IdentityMiddleware::default())
			.service(
				web::resource("/health")
				.route(web::get().to(|| async { HttpResponse::Ok().json("I am alive!")})),
			)
			.service(
				web::scope("/api")
				.configure(api::auth::init)
				.configure(api::client::init)
			)
	})
    .bind("127.0.0.1:8080")
	.expect("Failed to bind to port 8080")
    .run()
    .await
}

fn setup_oauth_client() -> BasicClient
{
	let client_id = 
		ClientId::new(dotenvy::var("CLIENT_ID")
		.expect("REDIRACT_URI not set."));
    let client_secret = 
		ClientSecret::new(dotenvy::var("CLIENT_SECRET")
		.expect("REDIRACT_URI not set."));
    let auth_url = 
		AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())
		.expect("Invalid authorization endpoint URL");
    let token_url = 
		TokenUrl::new("https://api.intra.42.fr/oauth/token".to_string())
		.expect("Invalid token endpoint URL");
    let redirect_uri = 
		RedirectUrl::new(dotenvy::var("REDIRECT_URI").
		expect("REDIRACT_URI not set."))
		.expect("Invalid redirect URL");

    BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_uri) // The 42API doesn't support token revocation.
}