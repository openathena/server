use std::sync::{Arc, Mutex};
use websocket::server::Server;
use std::thread;
use game::Game;
use ws;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender as ChannelSender;
use ws::Sender as WsSender;
use std::time::Duration;

use websocket::request::Request as WsRequest;
use serde_json;


struct Handler {
	channel_sender: ChannelSender<()>,
	ws_sender: WsSender
}

impl ws::Handler for Handler {
	fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
		self.ws_sender.send(ws::Message::from(
			serde_json::to_string(
				&WsRequest::Observe
			).unwrap()
		)).unwrap();
		Ok(())
	}
	fn on_message(&mut self, _msg: ws::Message) -> ws::Result<()> {
		self.channel_sender.send(()).unwrap();
		Ok(())
	}
}

#[test]
fn test() {
	let game = Arc::new(Mutex::new(Game::new()));

	let (send, recv) = channel();


	let _server_thread = thread::spawn(move || {
		Server::new(game).start();
	});

	thread::sleep_ms(1000);//TODO: wait for server to start

	let _client_thread = thread::spawn(move || {
		ws::connect("ws://127.0.0.1:43202", |out| {
			Handler {
				channel_sender: send.clone(),
				ws_sender: out
			}
		}).unwrap();
	});

	recv.recv_timeout(Duration::from_secs(1)).unwrap();
}