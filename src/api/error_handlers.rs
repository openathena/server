use rocket::{self, Catcher, Request};
use rocket::http::Status;
use rocket_contrib::Json;
use rocket::response::{Responder, Response};
use rocket::response::status::Custom as CustomStatus;
use serde_json::Error as JsonError;

const INTERNAL_SERVER_ERROR_MSG: &'static str = "A unexpected error occurred processing your request.";

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorType {
	ValidationFailed,
	BadRequest,
	NotFound,
	InternalServerError,
	Unauthorized,
	Forbidden
}

impl ApiErrorType {
	pub fn get_status(&self) -> Status {
		match *self {
			ApiErrorType::ValidationFailed => Status::BadRequest,
			ApiErrorType::BadRequest => Status::BadRequest,
			ApiErrorType::NotFound => Status::NotFound,
			ApiErrorType::InternalServerError => Status::InternalServerError,
			ApiErrorType::Unauthorized => Status::Unauthorized,
			ApiErrorType::Forbidden => Status::Forbidden
		}
	}
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
	error_type: ApiErrorType,
	description: String,
}

impl ApiError {
	pub fn new(error_type: ApiErrorType, description: &str) -> ApiError {
		ApiError {
			error_type,
			description: description.to_owned(),
		}
	}
}

impl From<JsonError> for ApiError {
	fn from(_err: JsonError) -> Self {
		ApiError::new(ApiErrorType::InternalServerError, INTERNAL_SERVER_ERROR_MSG)
	}
}

impl<'r> Responder<'r> for ApiError {
	fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
		CustomStatus(self.error_type.get_status(), Json(self)).respond_to(request)
	}
}

#[error(400)]
fn catch400(_req: &Request) -> ApiError {
	ApiError::new(ApiErrorType::BadRequest, "That's a bad request. That's all I can tell you right now.")
}

#[error(404)]
fn catch404(_req: &Request) -> ApiError {
	ApiError::new(ApiErrorType::NotFound, "The requested resource was not found.")
}

#[error(500)]
fn catch500(_req: &Request) -> ApiError {
	ApiError::new(ApiErrorType::InternalServerError, "I failed. I'll try harder next time")
}

pub fn get_catchers() -> Vec<Catcher> {
	errors![catch400, catch404, catch500]
}