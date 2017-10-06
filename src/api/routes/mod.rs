pub mod team;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
	team::get_routes()
}