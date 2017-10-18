use game::hex_grid::*;
use game::Game;

#[derive(Clone)]
pub struct Submarine {
	id: String,
	coords: Coordinate,
	team_id: String
}

impl Submarine {
	pub fn new<T: Into<Coordinate>>(coords: T, team_id: &str) -> Submarine {
		Submarine {
			id: Game::generate_id(),
			coords: coords.into(),
			team_id: team_id.to_owned()
		}
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}

	pub fn get_coords(&self) -> Coordinate {
		self.coords
	}

	pub fn get_team_id(&self) -> String {
		self.team_id.to_owned()
	}
}