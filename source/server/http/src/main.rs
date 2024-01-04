mod api;
mod chat;
mod db;
mod oauth;

use actix::Actor;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie, http::header, middleware::Logger, web, App, HttpResponse, HttpServer};
use oauth2::basic::BasicClient;

use crate::api::{auth, room, user, ws};

#[actix_web::main]
async fn main() {
    let db = db::Database::new();

    let auth_client = oauth::setup_oauth_client();

    let chat_server = chat::ChatServer::new().start();

    // get cookie key from enviroment
    let env_key = std::env::var("SESSION_KEY").expect("SESSION_KEY must be set");
    let secret_key = cookie::Key::from(env_key.as_bytes());

    println!(" < This works >");

    // Start the Actix Web server
    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .app_data(web::Data::new(auth_client.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::builder().build())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .service(
                web::resource("/health")
                    .route(web::get().to(|| async { HttpResponse::Ok().json("I am alive!") })),
            )
            // home
            .service(user::home)
            .service(user::clear)
            // authentication
            .service(auth::login)
            .service(auth::logout)
            .service(auth::callback)
            .service(auth::check)
            // user
            .service(user::all)
            .service(user::get)
            .service(user::post)
            .service(user::rooms)
            // room
            .service(room::all)
            .service(room::get)
            .service(room::list)
            .service(room::messages)
            .service(room::create)
            .service(room::update)
            .service(room::personal)
            .service(room::join)
            .service(room::part)
            // chat
            .service(ws::server)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind("0.0.0.0:8080")
    .expect("Failed to bind to port 8080")
    .run()
    .await;
}
