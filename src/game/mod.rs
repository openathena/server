use std::collections::HashMap;

mod team;
mod world;
pub mod actions;
pub mod events;

use self::actions::Action;
use self::events::*;
pub use self::team::Team;
use self::world::World;
use std::sync::Arc;

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
			world: World::new(16, 16),
			event_history: Vec::new(),
			event_listeners: Vec::new()
		}
	}

	pub fn execute_action(&mut self, action: Action) -> Result<(), ()> {
		match action.clone() {
			Action::CreateTeam(info) => {
				let team = Team::new(info.clone());
				self.teams.insert(team.get_id().to_owned(), team);
				self.generate_event(Visibility::Public, Event::TeamCreated(TeamCreated {
					id: info.id,
					name: info.name,
				}));
				Ok(())
			}
		}
	}

	fn generate_event(&mut self, visibility: Visibility, event: Event) {
		let visible_event = VisibleEvent::new(visibility, event);

		for listener in &mut self.event_listeners {
			listener(visible_event.clone())
		}

		self.event_history.push(visible_event);
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