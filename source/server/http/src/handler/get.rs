
use crate::handler::*;

pub fn handle(path: &str) -> HttpResponse {

	println!("default GET REQUEST on {}", path);

	match path {
		"/" => {
			// Execute code for the root path on GET request
			HttpResponse::Ok().body("Root endpoint")
		}
		"/clients/" => {
			let mut appendix = String::from("");
			if let Some(dynamic_part) = path.strip_prefix("/clients/") {
				appendix = dynamic_part.to_string();
			}

			HttpResponse::Ok().body(appendix)
			// Execute code for the "/clients/info" path on GET request
			// HttpResponse::Ok().body("Client info endpoint")
		}
		// "/clients/details" => {
		// 	// Execute code for the "/clients/details" path on GET request
		// 	HttpResponse::Ok().body("Client details endpoint")
		// }
		_ => {
			// Handle unrecognized paths on GET request
			HttpResponse::NotFound().body("Not Found")
		}
	}
}