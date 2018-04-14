use hex_grid::*;
use game::state::GameState;
use std::time::Duration;
use game::server_time::ServerTime;
use events::SubmarineUpdated;

#[derive(Clone)]
pub struct Submarine {
	id: String,
	coords: Coordinate,
	team_id: String,
	move_cooldown_end: ServerTime,
	torpedo_cooldown_end: ServerTime,
	health: u32,
}

impl Submarine {
	const STARTING_HEALTH: u32 = 3;

	pub fn new<T: Into<Coordinate>>(coords: T, team_id: &str) -> Submarine {
		Submarine {
			id: GameState::generate_id(),
			coords: coords.into(),
			team_id: team_id.to_owned(),
			move_cooldown_end: ServerTime::zero(),
			torpedo_cooldown_end: ServerTime::zero(),
			health: Self::STARTING_HEALTH,
		}
	}

	pub fn move_cooldown() -> Duration {
		Duration::from_secs(10)
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}

	pub fn get_coords(&self) -> Coordinate {
		self.coords
	}

	pub fn get_move_cooldown_end(&self) -> ServerTime {
		self.move_cooldown_end.clone()
	}

	pub fn move_to(&mut self, dest: Coordinate, cooldown_time: ServerTime) {
		self.coords = dest;
		self.move_cooldown_end = cooldown_time;
	}

	pub fn get_updated_event(&self) -> SubmarineUpdated {
		SubmarineUpdated {
			x: self.coords.x,
			y: self.coords.y,
			submarine_id: self.id.clone(),
			team_id: self.team_id.clone(),
			move_cooldown: self.move_cooldown_end.get_millis(),
			health: self.health,
		}
	}
}