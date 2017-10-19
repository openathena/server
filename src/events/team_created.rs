use super::EventType;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamCreated {
	pub id: String,
	pub name: String
}

impl EventType for TeamCreated {
	const TYPE: &'static str = "TEAM_CREATED";
}

