pub mod team;
pub mod world;
pub mod events;
pub mod hex_grid;
pub mod auth;

use self::events::*;
pub use self::team::Team;
use self::world::World;
pub use self::auth::AuthType;
use api::error_handlers::{ApiError, ApiErrorType};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct Game {
	teams: HashMap<String, Team>,
	world: World,
	event_history: Vec<VisibleEvent>,
	event_listeners: HashMap<String, Box<FnMut(VisibleEvent) + Send>>
}

impl Game {
	pub fn new() -> Game {
		Game {
			teams: HashMap::new(),
			world: World::new(8),
			event_history: Vec::new(),
			event_listeners: HashMap::new()
		}
	}

	fn generate_event(&mut self, visibility: Visibility, event: Event) {
		let visible_event = VisibleEvent::new(visibility, event);

		for listener in &mut self.event_listeners.values_mut() {
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

	pub fn auth(&self, credentials: Option<(&str, &str)>) -> Result<AuthType, ApiError> {
		if let Some((username, password)) = credentials {
			if let Some(team) = self.get_team(username) {
				if team.check_password(password) {
					Ok(AuthType::Team(team.get_id()))
				} else {
					Err(ApiError::new(ApiErrorType::Unauthorized, "Invalid username or password"))
				}
			} else {
				Err(ApiError::new(ApiErrorType::Unauthorized, "Invalid username or password"))
			}
		} else {
			Ok(AuthType::Observer)
		}
	}

	pub fn get_team(&self, id: &str) -> Option<&Team> {
		self.teams.get(id)
	}

	pub fn add_event_listener<F>(&mut self, listener: F) -> String
		where F: FnMut(VisibleEvent) + Send + 'static {
		let id = Self::generate_id();
		self.event_listeners.insert(id.clone(), Box::new(listener));
		id
	}

	pub fn remove_event_listener(&mut self, id: &str) {
		self.event_listeners.remove(id);
	}

	pub fn get_event_history(&self) -> &Vec<VisibleEvent> {
		&self.event_history
	}

	fn generate_id() -> String {
		thread_rng()
				.gen_ascii_chars()
				.take(24)
				.collect::<String>()
	}
}