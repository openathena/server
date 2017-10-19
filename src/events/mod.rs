pub mod team_created;
pub mod tile_updated;
pub mod submarine_created;
pub mod game_started;

pub use self::team_created::TeamCreated;
pub use self::tile_updated::TileUpdated;
pub use self::submarine_created::SubmarineCreated;
pub use self::game_started::GameStarted;

use serde::Serialize;
use serde_json;
use serde_json::Value as JsonValue;
use api::error_handlers::ApiError;
use std::time::Duration;

const MILLIS_PER_SEC: u64 = 1000;
const NANOS_PER_MILLI: u32 = 1000_000;

#[derive(Clone, PartialEq)]
pub enum Visibility {
	Public,
	Team(String)
}

#[derive(Clone)]
pub struct VisibleEvent {
	visibility: Visibility,
	event: Event
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
	data: JsonValue
}

impl Event {
	pub fn new<T: EventType>(event_type: &T, duration: Duration) -> Result<Event, ApiError> {
		let millis = (duration.as_secs() * MILLIS_PER_SEC) + (duration.subsec_nanos() / NANOS_PER_MILLI) as u64;
		Ok(Event {
			server_time: millis,
			event_type: T::TYPE.to_owned(),
			data: serde_json::to_value(event_type)?,
		})
	}
}

pub trait EventType: Serialize {
	const TYPE: &'static str;
}


