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

use api::server::ApiServer;

use game::Game;
use std::sync::{Arc, Mutex};
use rocket::Request;
use rocket_contrib::Json;
use std::thread;


fn main() {
	let game = Arc::new(Mutex::new(Game::new()));

	let server_thread = thread::spawn(move || {
		ApiServer::new(game.clone()).start();
	});

	server_thread.join();
}