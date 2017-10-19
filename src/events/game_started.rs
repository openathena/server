use super::EventType;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStarted;

impl EventType for GameStarted {
	const TYPE: &'static str = "GAME_STARTED";
}
