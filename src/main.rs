#![feature(plugin)]
#![plugin(rocket_codegen)]
//#![feature(const_atomic_bool_new)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate rand;

mod game;
mod api;

use game::Game;
use std::sync::{Arc, Mutex};
use rocket::Request;
use rocket_contrib::Json;

#[derive(Serialize)]
struct ErrorResponse{
	error_type: String,
	message: String
}
impl ErrorResponse{
	pub fn new(err_type: &str, msg: &str) -> ErrorResponse{
		ErrorResponse{
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

fn main() {
	let game = Arc::new(Mutex::new(Game::new()));

	rocket::ignite()
			.mount("/api/", api::get_routes())
			.catch(errors![catch404, catch500])
			.manage(game)
			.launch();
}