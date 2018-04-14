pub mod auth;
pub mod team;
pub mod submarine;
pub mod server_time;
pub mod state;

use self::state::GameState;
use events::*;
pub use self::team::Team;
pub use self::auth::AuthType;
use api::error_handlers::{ApiError, ApiErrorType};
use std::collections::HashMap;
use hex_grid::*;
use std::time::Instant;
use self::server_time::ServerTime;
use std::sync::{Arc, Mutex};
use self::state::*;

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

#[derive(Clone)]
pub struct Game {
	state: Arc<Mutex<GameState>>,
}

impl Game {
	pub fn new() -> Game {
		Game {
			state: Arc::new(Mutex::new(GameState {
				game_start: Instant::now(),
				state: State::TeamCreation,
				teams: HashMap::new(),
				world: HashMap::new(),
				event_history: Vec::new(),
				event_listeners: HashMap::new(),
				submarines: HashMap::new(),
			})),
		}
	}

	pub fn modify_state<T, F: FnOnce(&mut GameState) -> T>(&self, func: F) -> T {
		func(&mut self.state.lock().unwrap())
	}

	pub fn iter_event_history<F: FnMut(&VisibleEvent)>(&self, mut func: F) {
		let data = self.state.lock().unwrap();
		for event in &data.event_history {
			func(&event)
		}
	}
}