use ws;
use game::Game;
use std::sync::{Arc, Mutex};

const PORT: u16 = 43202;

pub struct WsServer {
	game: Arc<Mutex<Game>>
}

impl WsServer {
	pub fn new(game: Arc<Mutex<Game>>) -> WsServer {
		WsServer { game }
	}

	pub fn start(self) {
		ws::listen(("0.0.0.0", PORT), |out| {
			move |msg| {
				//TODO: read AUTH request here
				Ok(())
			}
		}).unwrap();
	}
}