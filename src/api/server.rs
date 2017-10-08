use std::thread;

use std::sync::{Arc, Mutex};
use game::Game;
use super::error_handlers;
use api;
use rocket;

pub struct Server {
	game: Arc<Mutex<Game>>
}

impl Server {
	pub fn new(game: Arc<Mutex<Game>>) -> Server {
		Server { game }
	}
	pub fn start(self) {
		rocket::ignite()
				.mount("/api/", api::routes::get_routes())
				.catch(error_handlers::get_catchers())
				.manage(self.game)
				.launch();
	}
}