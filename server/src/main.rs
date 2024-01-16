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
use env_logger::Env;
use log::info;

use crate::api::{auth, room, user};

/*
    find out how to handle one on one chat
*/

#[actix_web::main]
async fn main() {
    env_logger::init();

    let db = db::Database::new();

    let auth_client = oauth::setup_oauth_client();

    let chat_server = chat::ChatServer::new(db.clone()).start();

    let matchmaking_server = game::matchmake::MatchmakingServer::new().start();
    let tournament_server = game::tournament::TournamentServer::new().start();
    let one_vs_one_server = game::one_vs_one::OneVsOneServer::new().start();

    // get cookie key from enviroment
    let env_key = std::env::var("SESSION_KEY").expect("SESSION_KEY must be set");
    let secret_key = cookie::Key::from(env_key.as_bytes());

    println!(" < --- * --- >");

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
            .service(api::chat::server)
            //  game
            .service(api::game::matchmaking)
            .service(api::game::create_tournament)
            .service(api::game::connect_tournament)
            .service(api::game::one_vs_one)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind("0.0.0.0:8080")
    .expect("Failed to bind to port 8080")
    .run()
    .await;
}