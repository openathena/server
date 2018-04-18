pub mod create_team;
pub mod start_game;
pub mod move_submarine;
pub mod ping;

use serde_json::Value;
use serde_json;
use game::Game;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use api::error_handlers::ApiError;
use validate::Validate;
use api::error_handlers::ApiErrorType;


pub type ActionMap = HashMap<String, &'static Action>;

pub fn action_list() -> Vec<&'static Action> {
	vec![
		&create_team::Definition,
		&start_game::Definition,
		&move_submarine::Definition,
		&ping::Definition,
	]
}

pub fn action_map() -> ActionMap {
	action_list().into_iter().map(|x| (x.name(), x)).collect()
}


pub trait Action: Sync {
	fn name(&self) -> String;
	fn process(&self, json: Value, game: &mut Game) -> Result<Value, ApiError>;
}

pub trait ActionDef {
	const NAME: &'static str;
	type Request: for<'a> Deserialize<'a> + Validate;
	type Response: Serialize;

	fn execute(&self, request: Self::Request, game: &mut Game) -> Result<Self::Response, ApiError>;
}

impl<T: ActionDef + Sync> Action for T {
	fn name(&self) -> String {
		Self::NAME.to_owned()
	}

	fn process(&self, json: Value, game: &mut Game) -> Result<Value, ApiError> {
		let request: T::Request = serde_json::from_value(json).map_err(|_| {
			ApiError::new(ApiErrorType::BadRequest, "Unable to deserialize JSON request. Check your request format.")
		})?;
		if let Err(err) = request.validate() {
			return Err(ApiError::new(ApiErrorType::ValidationFailed, &err.get_message()));
		}
		let response = self.execute(request, game)?;
		Ok(serde_json::to_value(&response)?)
	}
}