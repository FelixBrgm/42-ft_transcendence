mod api;
mod chat;
mod db;
mod game;
mod oauth;

use actix::Actor;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie, http::header, middleware::Logger, web, App, HttpResponse, HttpServer};
use std::io::BufReader;
use std::fs::File; 

use crate::api::{auth, block, friend, user};

#[actix_web::main]
async fn main() {
    let db = db::Database::new();

    let auth_client = oauth::setup_oauth_client();

    let chat_server = chat::server::ChatServer::new(db.clone()).start();

    let matchmaking_server = game::matchmake::MatchmakingServer::new(db.clone()).start();
    let tournament_server = game::tournament::TournamentServer::new(db.clone()).start();
    let one_vs_one_server = game::one_vs_one::OneVsOneServer::new(db.clone()).start();

    // get cookie key from enviroment
    let env_key = std::env::var("SESSION_KEY").expect("SESSION_KEY must be set");
    let secret_key = cookie::Key::from(env_key.as_bytes());

    let mut certs_file = BufReader::new(File::open("../etc/nginx/ssl/ssl_final_cert.crt").expect("Couldn't find ssl.crt"));
    let mut key_file = BufReader::new(File::open("/etc/nginx/ssl/ssl_priv_key.key").expect("Couldn't fild ssl.key"));

    // load TLS certs and key
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let tls_certs = rustls_pemfile::certs(&mut certs_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
        .next()
        .unwrap()
        .unwrap();

    // set up TLS config options
    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
        .unwrap();

    // Start the Actix Web server
    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .app_data(web::Data::new(matchmaking_server.clone()))
            .app_data(web::Data::new(tournament_server.clone()))
            .app_data(web::Data::new(one_vs_one_server.clone()))
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
                web::resource("/")
                    .route(web::get().to(|| async { HttpResponse::Ok().json("I am alive!") })),
            )
            // authentication
            .service(auth::login)
            .service(auth::logout)
            .service(auth::callback)
            .service(auth::check)
            // user
            .service(user::get)
            .service(user::post)
            .service(user::find)
            .service(user::list)
            // friend
            .service(friend::toggle)
            .service(friend::list)
            .service(friend::check)
            // block
            .service(block::toggle)
            .service(block::check)
            // chat
            .service(api::chat::server)
            .service(api::chat::join_chat)
            .service(api::chat::get_rooms)
            .service(api::chat::get_messages_by_room_id)
            // //  game
            .service(api::game::matchmaking)
            .service(api::game::create_tournament)
            .service(api::game::connect_tournament)
            .service(api::game::one_vs_one)
            .service(api::game::list)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind_rustls_0_22("0.0.0.0:8080", tls_config)
    .expect("Failed to bind to port 8080")
    .run()
    .await;
}
