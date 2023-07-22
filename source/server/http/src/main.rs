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
use actix_web::{cookie::Key};
use actix_identity::IdentityMiddleware;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::url::Url;
use oauth2::StandardTokenResponse;

fn setup_database() -> Database
{
	dotenvy::from_filename("usr/src/.env").ok();
	// retrieve the POSTGRES_URL
	let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
	// create new Database
	let db = Database::new(&database_url);

    println!("Database connection established");

	db
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "debug");
	std::env::set_var("RUST_BACKTRACE", "1");
	env_logger::init();

	let db = setup_database();

    // Start the Actix Web server
	HttpServer::new( move || {

			// cookie::key
			// auth client
			let auth_client = authorize_client();
			println!("new client authorized");
			// cors
			App::new()
			.app_data(web::Data::new(db.clone()))
			.app_data(web::Data::new(auth_client))
			.wrap(IdentityMiddleware::default())
			.wrap(Logger::default())
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
    .bind("127.0.0.1:8080") // Bind the server to localhost on port 8080
	.expect("Failed to bind to port 8080")
    .run()
    .await
}

// i want to add a rickroll with external_resource in app


fn authorize_client() -> oauth2::basic::BasicClient {

	   oauth2::basic::BasicClient::new(
        oauth2::ClientId::new(
			dotenvy::var("CLIENT_ID").expect("CLIENT_ID is not set."),
        ),
        Some(oauth2::ClientSecret::new(
            dotenvy::var("CLIENT_SECRET").expect("CLIENT_SECRET is not set."),
        )),
        oauth2::AuthUrl::new(
			dotenvy::var("AUTHORIZATION_ENDPOINT").expect("AUTHORIZATION_ENDPOINT not set.")
		)
        .expect("Invalid authorization endpoint URL (AUTH_URL)"),
        Some(
            oauth2::TokenUrl::new(
				dotenvy::var("TOKEN_ENDPOINT").expect("TOKEN_ENDPOINT not set.")
			)
            .expect("Invalid token endpoint URL (TOKEN_URL)"),
        ),
    )
    .set_redirect_uri(
        oauth2::RedirectUrl::new(
			dotenvy::var("REDIRECT_URI").expect("REDIRACT_URL not set.")
        )
        .expect("Invalid redirect URL"),
    )
}