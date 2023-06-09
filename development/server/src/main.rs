use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// Define your API endpoint handler
#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix web server
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind("127.0.0.1:8000")? // Specify the IP address and port
    .run()
    .await
}
