#[derive(Clone, PartialEq)]
pub enum Visibility {
	Public
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
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Event {
	TeamCreated(TeamCreated)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamCreated {
	pub id: String,
	pub name: String
}