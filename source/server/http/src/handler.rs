
pub mod get;
pub mod post;

use actix_web::{web, HttpRequest, HttpResponse, Responder, http::Method};

pub async fn handle_request(req: HttpRequest) -> impl Responder {
	// Get the requested path
    let path = req.path();

    match req.method() {
        // Handle GET requests
        &Method::GET => { get::handle(path) },
        // Handle POST requests
		&Method::POST => { post::handle(path) },
        // Handle other HTTP methods
        _ => {
            HttpResponse::MethodNotAllowed().body("Method Not implemented yet")
        }
    }
}

pub async fn get_request(params: web::Path<(String, String)>) -> HttpResponse
{
	let table = &params.0;
	let data = &params.1;
	println!("params: {:?}", params);

	HttpResponse::MethodNotAllowed().body("GET REQUEST")
}