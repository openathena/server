use super::EventType;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmarineCreated {
	pub x: i32,
	pub y: i32,
	pub id: String,
	pub team_id: String
}

impl EventType for SubmarineCreated {
	const TYPE: &'static str = "SUBMARINE_CREATED";
}