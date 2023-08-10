use crate::http::db::Database;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use oauth2::basic::BasicClient;

mod auth;
mod error;
mod user;

async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome home!")
}

pub async fn start_actix_server(db: Database, auth_client: BasicClient) {
    // get cookie key from enviroment
    let env_key = std::env::var("SESSION_KEY").expect("SESSION_KEY must be set");
    let secret_key = cookie::Key::from(env_key.as_bytes());

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
            .app_data(web::Data::new(auth_client.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .route("/", web::get().to(home))
            .service(
                web::resource("/health")
                    .route(web::get().to(|| async { HttpResponse::Ok().json("I am alive!") })),
            )
            .service(
                web::scope("/api")
                    .configure(auth::init)
                    .configure(user::init),
            )
    })
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to port 8080")
    .run()
    .await;
}
