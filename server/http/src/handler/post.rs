
use crate::handler::*;

pub fn handle(path: &str) -> HttpResponse {

	println!("POST REQUEST on {}", path);

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