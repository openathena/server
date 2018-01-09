use game;
use super::EventType;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TileUpdated {
	pub x: i32,
	pub y: i32,
	pub tile: Tile
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
	#[serde(rename = "type")]
	pub tile_type: game::data::TileType
}

impl EventType for TileUpdated {
	const TYPE: &'static str = "TILE_UPDATED";
}