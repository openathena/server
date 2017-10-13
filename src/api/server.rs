use std::sync::{Arc, Mutex};
use game::Game;
use super::error_handlers;
use api;
use rocket;
use actions::action_map;

pub struct Server {
	game: Arc<Mutex<Game>>
}

impl Server {
	pub fn new(game: Arc<Mutex<Game>>) -> Server {
		Server { game }
	}
	pub fn start(self) {
		let action_map = action_map();
		rocket::ignite()
				.mount("/rpc/", routes![api::route::rpc_request])
				.catch(error_handlers::get_catchers())
				.manage(self.game)
				.manage(action_map)
				.launch();
	}
}