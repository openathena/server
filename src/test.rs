use std::sync::{Arc, Mutex};
use websocket::server::Server;
use std::thread;
use game::Game;
use ws;
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;

struct Handler{
	sender: Sender<()>
}

impl ws::Handler for Handler{
	fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
		self.sender.send(()).unwrap();
		Ok(())
	}
}

#[test]
fn test() {
	let game = Arc::new(Mutex::new(Game::new()));

	let _server_thread = thread::spawn(move || {
		Server::new(game).start();
	});

	let (send, recv) = channel();


	let _client_thread = thread::spawn(move || {
		ws::connect("ws://127.0.0.1:43202", |_out| {
			Handler {
				sender: send.clone()
			}
		}).unwrap();
	});

	recv.recv_timeout(Duration::from_secs(1)).unwrap();
}