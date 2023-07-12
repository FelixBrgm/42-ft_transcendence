
pub mod get;
pub mod post;

use actix_web::{HttpRequest, HttpResponse, Responder, http::Method};

pub async fn handle_request(req: HttpRequest) -> impl Responder {
    let path = req.path(); // Get the requested path

    match req.method() {
        // Handle GET requests
        &Method::GET => { get::handle(path) },
        // Handle other HTTP methods
        _ => {
            HttpResponse::MethodNotAllowed().body("Method Not implemented yet")
        }
    }
}