use rocket;
use game::Game;
use api::{error_handlers, route, cors};
use actions::action_map;

pub struct Server {
	game: Game
}

impl Server {
	pub fn new(game: Game) -> Server {
		Server { game }
	}
	pub fn start(self) {
		let action_map = action_map();
		let options = cors::options();

		rocket::ignite()
				.mount("/rpc/", routes![route::rpc_request])
				.attach(options)
				.catch(error_handlers::get_catchers())
				.manage(self.game)
				.manage(action_map)
				.launch();
	}
}
