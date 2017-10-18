use rocket_contrib::Json;
use rocket::State;
use std::sync::{Arc, Mutex};
use game::Game;
use super::error_handlers::ApiError;
use actions::ActionMap;
use serde_json::Value as JsonValue;

type GameMutex = Arc<Mutex<Game>>;

#[derive(Deserialize)]
pub struct RequestEnvelope {
	#[serde(rename = "type")]
	pub request_type: String,
	pub data: Option<JsonValue>
}

#[post("/", data = "<rpc_request>")]
pub fn rpc_request(rpc_request: Json<RequestEnvelope>, game: State<GameMutex>, action_map: State<ActionMap>)
                   -> Option<Result<Json<JsonValue>, ApiError>> {
	action_map.get(&rpc_request.request_type).map(|action| {
		let mut game = game.lock().unwrap();
		let request_data = rpc_request.data.clone().unwrap_or(JsonValue::Null);
		action.process(request_data, &mut game).map(|x| Json(x))
	})
}