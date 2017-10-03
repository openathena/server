use std::collections::HashMap;

mod team;
mod world;

pub use self::team::Team;
use self::world::World;

pub struct Game {
	teams: HashMap<String, Team>,
	world: World
}

impl Game {
	pub fn new() -> Game {
		Game {
			teams: HashMap::new(),
			world: World::new(16, 16)
		}
	}

	pub fn add_team(&mut self, team: Team) {
		self.teams.insert(team.get_id().to_owned(), team);
	}

	pub fn get_team(&self, id: &str) -> Option<&Team> {
		self.teams.get(id)
	}

	pub fn get_teams(&self) -> Vec<&Team> {
		self.teams.values().collect()
	}
}