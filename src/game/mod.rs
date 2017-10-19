pub mod hex_grid;
pub mod auth;
pub mod team;
pub mod submarine;

use events::*;
pub use self::team::Team;
pub use self::auth::AuthType;
use api::error_handlers::{ApiError, ApiErrorType};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use self::hex_grid::*;
use self::submarine::Submarine;
use std::time::Instant;

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
	game_start: Instant,
	state: State,
	teams: HashMap<String, Team>,
	world: HashMap<Coordinate, TileType>,
	event_history: Vec<VisibleEvent>,
	event_listeners: HashMap<String, Box<FnMut(VisibleEvent) + Send>>,
	submarines: HashMap<String, Submarine>
}

impl Game {
	pub fn new() -> Game {
		Game {
			game_start: Instant::now(),
			state: State::TeamCreation,
			teams: HashMap::new(),
			world: HashMap::new(),
			event_history: Vec::new(),
			event_listeners: HashMap::new(),
			submarines: HashMap::new()
		}
	}

	pub fn generate_event<T: EventType>(&mut self, visibility: Visibility, event_type: &T) -> Result<(), ApiError> {
		let visible_event = VisibleEvent::new(visibility, Event::new(event_type, self.game_start.elapsed())?);
		for listener in &mut self.event_listeners.values_mut() {
			listener(visible_event.clone())
		}
		self.event_history.push(visible_event);
		Ok(())
	}

	pub fn add_team(&mut self, team: Team) -> Result<(), ApiError> {
		self.state.require_state(State::TeamCreation)?;
		self.teams.insert(team.get_id().to_owned(), team.clone());
		self.generate_event(Visibility::Public, &TeamCreated {
			id: team.get_id(),
			name: team.get_name(),
		})?;
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
		self.generate_world()?;
		self.place_starting_submarines()?;
		self.generate_event(Visibility::Public, &GameStarted)?;
		self.state = State::Started;
		Ok(())
	}

	fn generate_world(&mut self) -> Result<(), ApiError> {
		let default_tile = TileType::Water;

		//TODO: not sure what the size/shape should be yet
		//TODO: make sure map is large enough to fit ships for all teams
		let coords = CENTER + Offset::fill_hex(8);
		for coord in coords {
			self.world.insert(coord, default_tile);
			self.generate_event(Visibility::Public, &TileUpdated {
				x: coord.x,
				y: coord.y,
				tile: tile_updated::Tile { tile_type: default_tile },
			})?;
		}
		Ok(())
	}

	fn place_starting_submarines(&mut self) -> Result<(), ApiError> {
		let mut empty_tiles: Vec<Coordinate> = self.world.keys()
				.map(|x| x.clone())
				.filter(|x| self.is_tile_empty(*x))
				.collect();
		thread_rng().shuffle(&mut empty_tiles);
		let teams: Vec<Team> = self.teams.values().map(|x| x.clone()).collect();
		for team in teams {
			let coord = if let Some(coord) = empty_tiles.pop() {
				coord
			} else {
				return Err(ApiError::new(ApiErrorType::InternalServerError, "Not enough tiles for ships"));
			};
			let submarine = Submarine::new(coord, &team.get_id());
			self.submarines.insert(submarine.get_id(), submarine.clone());
			self.generate_event(Visibility::Team(team.get_id()), &SubmarineCreated {
				x: coord.x,
				y: coord.y,
				id: submarine.get_id(),
				team_id: submarine.get_team_id(),
			})?;
		}
		Ok(())
	}

	pub fn is_tile_empty<T: Into<Coordinate>>(&self, coord: T) -> bool {
		self.get_submarine_at(coord).is_none()
	}

	pub fn get_submarine_at<T: Into<Coordinate>>(&self, coords: T) -> Option<&Submarine> {
		let coords = coords.into();
		self.submarines.values().find(move |x| x.get_coords() == coords)
	}

	fn generate_id() -> String {
		thread_rng()
				.gen_ascii_chars()
				.take(24)
				.collect::<String>()
	}
}