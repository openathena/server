#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rocket_contrib;
extern crate rocket;
extern crate rocket_cors;
extern crate rand;
extern crate ws;
extern crate serde_json;
extern crate validate;
extern crate serde;
extern crate hex_grid;

#[cfg(test)]
mod test;

mod game;
mod api;
mod websocket;
mod actions;
mod events;

use api::server::Server as ApiServer;
use websocket::server::Server as WsServer;

use game::Game;
use std::thread;


fn main() {
	let game = Game::new();

	let api_thread = {
		let game = game.clone();
		thread::spawn(move || {
			ApiServer::new(game).start();
		})
	};


	let ws_thread = thread::spawn(move || {
		WsServer::new(game).start();
	});

	api_thread.join().unwrap();
	ws_thread.join().unwrap();
}
