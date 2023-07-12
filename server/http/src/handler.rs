use actix_web::{HttpRequest, HttpResponse, Responder, http::Method};

pub mod get;
pub mod post;

pub async fn handle_request(req: HttpRequest) -> impl Responder {
    let path = req.path(); // Get the requested path

    match req.method() {
        // Handle GET requests
        &Method::GET => {
            // Perform logic for GET requests
            match path {
                "/" => {
                    // Execute code for the root path on GET request
                    HttpResponse::Ok().body("Root endpoint")
                }
                "/clients/info" => {
                    // Execute code for the "/clients/info" path on GET request
                    HttpResponse::Ok().body("Client info endpoint")
                }
                "/clients/details" => {
                    // Execute code for the "/clients/details" path on GET request
                    HttpResponse::Ok().body("Client details endpoint")
                }
                _ => {
                    // Handle unrecognized paths on GET request
                    HttpResponse::NotFound().body("Not Found")
                }
            }
        }
        // Handle POST requests
        &Method::POST => {
            // Perform logic for POST requests
            match path {
                "/clients/create" => {
                    // Execute code for the "/clients/create" path on POST request
                    HttpResponse::Ok().body("Create client endpoint")
                }
                _ => {
                    // Handle unrecognized paths on POST request
                    HttpResponse::NotFound().body("Not Found")
                }
            }
        }
        // Handle other HTTP methods
        _ => {
            HttpResponse::MethodNotAllowed().body("Method Not Allowed")
        }
    }
}