use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ApiError {
	#[display(fmt = "Unauthorized")]
    Unauthorized,
    #[display(fmt = "Not Found")]
    NotFound,
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Internal Server Error: {}", _0)]
    InternalServerError(String),
}

// Implement ResponseError trait for ApiError to return a Http response
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
            ApiError::NotFound => HttpResponse::NotFound().body("Not Found"),
			ApiError::BadRequest(message) => {
                HttpResponse::BadRequest().body(message.to_string())
            }
            ApiError::InternalServerError(message) => {
                HttpResponse::InternalServerError().body(message.to_string())
            }
        }
    }
}