use tokio::sync::mpsc;

mod chat;
mod http;

use chat::RoomSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (room_update_sender, room_update_receiver) = mpsc::channel::<chat::RoomSocket>(100);
    
    {
        let room_update_sender = room_update_sender.clone();
        let _ = tokio::spawn(chat::start_chat_server(
            room_update_sender,
            room_update_receiver,
        ));
    }

    http::start_actix_server(room_update_sender).await;

    Ok(()) // only so no error hehe
}
// // #[macro_use]
// // extern crate diesel;

// #![allow(dead_code)]
// #![allow(unused_imports)]
// mod api;
// mod chat;
// #[allow(unused_variables)]
// mod db;

// use actix_cors::Cors;
// use actix_identity::IdentityMiddleware;
// use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
// use actix_web::middleware::Logger;
// use actix_web::{cookie, http::header};
// use actix_web::{get, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
// use chat::chat::start_chat_server;
// use chat::runtime::RoomSocket;
// use db::models::NewUser;
// use db::wrapper::Database;
// use std::time::Duration;
// use tokio::sync::mpsc::{self, Receiver, Sender};
// // use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// use oauth2::basic::BasicClient;
// use oauth2::reqwest::http_client;
// use oauth2::url::Url;
// use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, StandardTokenResponse, TokenUrl};

// async fn home() -> impl Responder {
//     HttpResponse::Ok().body("Welcome home!")
// }

// async fn test(
//     req: HttpRequest,
//     db: web::Data<Database>,
// ) -> Result<HttpResponse, crate::api::errors::ApiError> {
//     actix_identity::Identity::login(&req.extensions(), "1".to_string())?;
//     if let Err(_) = db.add_user(&NewUser {
//         id: 1,
//         login: "anna".to_string(),
//         avatar: "pb".to_string(),
//     }) {
//         println!("1 is already inside the db!");
//         return Ok(HttpResponse::Ok().body("Identity 1 is already initalized"));
//     }
//     println!("INITIALISED testing Identity 1");
//     Ok(HttpResponse::Ok().body("Initialised Identity 1"))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // std::env::set_var("RUST_LOG", "debug");
//     // std::env::set_var("RUST_BACKTRACE", "1");
//     env_logger::init();

//     // Initialize the logger with a specific log level
//     // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

//     let (room_update_sender, room_update_receiver) = mpsc::channel::<RoomSocket>(100);
//     {
//         let room_update_sender = room_update_sender.clone();
//         tokio::spawn(start_chat_server(room_update_sender, room_update_receiver));
//     }
//     println!("Chatserver started!");

//     let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
//     let db = Database::new(&database_url);
//     println!("Database connection established!");

//     let auth_client = setup_oauth_client();
//     println!("Authentication established!");

//     // cookie::key
//     let env_key = std::env::var("SESSION_KEY").expect("SESSION_KEY must be set");
//     let secret_key = cookie::Key::from(env_key.as_bytes());

//     // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
//     // builder
//     //     .set_private_key_file("key.pem", SslFiletype::PEM)
//     //     .expect("it failed here");
//     // builder.set_certificate_chain_file("cert.pem").expect("this failed");

//     // Start the Actix Web server
//     HttpServer::new(move || {
//         let cors = Cors::default()
//             .allow_any_origin()
//             .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
//             .allowed_headers(vec![
//                 header::CONTENT_TYPE,
//                 header::AUTHORIZATION,
//                 header::ACCEPT,
//             ])
//             .supports_credentials();

//         App::new()
//             .app_data(web::Data::new(db.clone()))
//             .app_data(web::Data::new(auth_client.clone()))
//             .wrap(cors)
//             .wrap(Logger::default())
//             .wrap(IdentityMiddleware::default())
//             .wrap(
//                 SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
//                     .cookie_secure(false)
//                     .build(),
//             )
//             .route("/", web::get().to(home))
//             .route("/test", web::get().to(test))
//             .service(
//                 web::resource("/health")
//                     .route(web::get().to(|| async { HttpResponse::Ok().json("I am alive!") })),
//             )
//             .service(
//                 web::scope("/api")
//                     .configure(api::auth::init)
//                     .configure(api::user::init),
//             )
//     })
//     // .bind_openssl("127.0.0.1:8080", builder)
//     .bind("127.0.0.1:8080")
//     .expect("Failed to bind to port 8080")
//     .run()
//     .await
// }

// // curl -X GET http://127.0.0.1:8080/health
