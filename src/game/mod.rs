pub mod events;
pub mod hex_grid;
pub mod auth;
pub mod team;

use self::events::*;
pub use self::team::Team;
pub use self::auth::AuthType;
use api::error_handlers::{ApiError, ApiErrorType};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use self::hex_grid::*;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TileType {
	Water
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
	TeamCreation,
	Started
}

impl State {
	pub fn require_state(&self, state: State) -> Result<(), ApiError> {
		if *self == state {
			Ok(())
		} else {
			let msg = &format!("Illegal game state for action. Required '{:?}' but is currently '{:?}'", state, *self);
			Err(ApiError::new(ApiErrorType::BadRequest, msg))
		}
	}
}

pub struct Game {
	state: State,
	teams: HashMap<String, Team>,
	world: HexGrid<TileType>,
	event_history: Vec<VisibleEvent>,
	event_listeners: HashMap<String, Box<FnMut(VisibleEvent) + Send>>
}

impl Game {
	pub fn new() -> Game {
		Game {
			state: State::TeamCreation,
			teams: HashMap::new(),
			world: HexGrid::new(),
			event_history: Vec::new(),
			event_listeners: HashMap::new()
		}
	}

	pub fn generate_event(&mut self, visibility: Visibility, event: Event) {
		let visible_event = VisibleEvent::new(visibility, event);

		for listener in &mut self.event_listeners.values_mut() {
			listener(visible_event.clone())
		}

		self.event_history.push(visible_event);
	}

	pub fn add_team(&mut self, team: Team) -> Result<(), ApiError> {
		self.state.require_state(State::TeamCreation)?;
		self.teams.insert(team.get_id().to_owned(), team.clone());
		self.generate_event(Visibility::Public, Event::TeamCreated(TeamCreated {
			id: team.get_id(),
			name: team.get_name(),
		}));
		Ok(())
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

	pub fn start(&mut self) -> Result<(), ApiError> {
		self.state.require_state(State::TeamCreation)?;
		self.generate_world();
		self.generate_event(Visibility::Public, Event::GameStarted);
		self.state = State::Started;
		Ok(())
	}

	fn generate_world(&mut self) {
		let default_tile = TileType::Water;
		let coords = CENTER + Offset::fill_hex(8);//TODO: not sure what the size/shape should be yet
		for coord in coords {
			self.world.set(coord, default_tile);
			self.generate_event(Visibility::Public, Event::TileUpdated(events::TileUpdated {
				x: coord.x,
				y: coord.y,
				tile: Tile { tile_type: default_tile },
			}))
		}
	}

	fn generate_id() -> String {
		thread_rng()
				.gen_ascii_chars()
				.take(24)
				.collect::<String>()
	}
}