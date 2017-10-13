use std::collections::HashMap;

pub mod team;
pub mod world;
pub mod events;
pub mod hex_grid;

use self::events::*;
pub use self::team::Team;
use self::world::World;

pub struct Game {
	teams: HashMap<String, Team>,
	world: World,
	event_history: Vec<VisibleEvent>,
	event_listeners: Vec<Box<FnMut(VisibleEvent) + Send>>
}

impl Game {
	pub fn new() -> Game {
		Game {
			teams: HashMap::new(),
			world: World::new(8),
			event_history: Vec::new(),
			event_listeners: Vec::new()
		}
	}

	fn generate_event(&mut self, visibility: Visibility, event: Event) {
		let visible_event = VisibleEvent::new(visibility, event);

		for listener in &mut self.event_listeners {
			listener(visible_event.clone())
		}

		self.event_history.push(visible_event);
	}

	pub fn add_team(&mut self, team: Team) {
		self.teams.insert(team.get_id().to_owned(), team.clone());
		self.generate_event(Visibility::Public, Event::TeamCreated(TeamCreated {
			id: team.get_id(),
			name: team.get_name(),
		}));
	}

	pub fn get_team(&self, id: &str) -> Option<&Team> {
		self.teams.get(id)
	}

	pub fn add_event_listener<F>(&mut self, listener: F)
		where F: FnMut(VisibleEvent) + Send + 'static {
		self.event_listeners.push(Box::new(listener));
	}

	pub fn get_event_history(&self) -> &Vec<VisibleEvent> {
		&self.event_history
	}
}