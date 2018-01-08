use super::EventType;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmarineMoveCooldown {
    pub submarine_id: String,
}

impl EventType for SubmarineMoveCooldown {
    const TYPE: &'static str = "SUBMARINE_MOVE_COOLDOWN";
}