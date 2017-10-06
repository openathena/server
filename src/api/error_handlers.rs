use rocket::{self, Catcher, Request, Route};
use rocket_contrib::Json;

#[derive(Serialize)]
struct ErrorResponse {
	error_type: String,
	message: String
}

impl ErrorResponse {
	pub fn new(err_type: &str, msg: &str) -> ErrorResponse {
		ErrorResponse {
			error_type: err_type.to_owned(),
			message: msg.to_owned()
		}
	}
}

#[error(404)]
fn catch404(req: &Request) -> Json<ErrorResponse> {
	Json(ErrorResponse::new("not_found", "The requested resource was not found"))
}

#[error(500)]
fn catch500(req: &Request) -> Json<ErrorResponse> {
	Json(ErrorResponse::new("internal_server_error", "I failed. I'll try harder next time"))
}

pub fn get_catchers() -> Vec<Catcher> {
	errors![catch404, catch500]
}