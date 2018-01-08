use super::EventType;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmarineMoved {
    pub x: i32,
    pub y: i32,
    pub submarine_id: String,
    pub move_cooldown: u64
}

impl EventType for SubmarineMoved {
    const TYPE: &'static str = "SUBMARINE_MOVED";
}