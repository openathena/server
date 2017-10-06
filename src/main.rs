#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate ws;

mod game;
mod api;
mod ws_server;

use api::server::ApiServer;
use ws_server::WsServer;

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