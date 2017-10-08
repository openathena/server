#![feature(plugin, conservative_impl_trait)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate ws;
extern crate serde_json;

mod game;
mod api;
mod websocket;

use api::server::Server as ApiServer;
use websocket::server::Server as WsServer;

use game::Game;
use std::sync::{Arc, Mutex};
use rocket::Request;
use rocket_contrib::Json;
use std::thread;


fn main() {
	let game = Arc::new(Mutex::new(Game::new()));

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