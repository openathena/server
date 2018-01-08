pub mod team_created;
pub mod tile_updated;
pub mod submarine_created;
pub mod submarine_moved;
pub mod submarine_move_cooldown;
pub mod game_started;


pub use self::team_created::TeamCreated;
pub use self::tile_updated::TileUpdated;
pub use self::submarine_created::SubmarineCreated;
pub use self::submarine_moved::SubmarineMoved;
pub use self::submarine_move_cooldown::SubmarineMoveCooldown;
pub use self::game_started::GameStarted;

use serde::Serialize;
use serde_json;
use serde_json::Value as JsonValue;
use api::error_handlers::ApiError;
use game::server_time::ServerTime;

#[derive(Clone, PartialEq)]
pub enum Visibility {
	Public,
	Team(String),
}

#[derive(Clone)]
pub struct VisibleEvent {
	visibility: Visibility,
	event: Event,
}

impl VisibleEvent {
	pub fn new(visibility: Visibility, event: Event) -> VisibleEvent {
		VisibleEvent { visibility, event }
	}
	pub fn get_visibility(&self) -> Visibility {
		self.visibility.clone()
	}
	pub fn get_event(&self) -> Event {
		self.event.clone()
	}
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
	#[serde(rename = "type")]
	event_type: String,
	server_time: u64,
	data: JsonValue,
}

impl Event {
	pub fn new<T: EventType>(event: &T, server_time: ServerTime) -> Result<Event, ApiError> {
		Ok(Event {
			server_time: server_time.get_millis(),
			event_type: T::TYPE.to_owned(),
			data: serde_json::to_value(event)?,
		})
	}
}

pub trait EventType: Serialize {
	const TYPE: &'static str;
}


