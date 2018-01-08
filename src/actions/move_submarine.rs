use super::ActionDef;
use api::error_handlers::ApiError;
use game::Game;
use validate::{Validate, ValidationResult};
use game::submarine::Submarine;

pub struct Definition;

impl ActionDef for Definition {
	const NAME: &'static str = "MOVE_SUBMARINE";
	type Request = Request;
	type Response = Response;

	fn execute(&self, request: Self::Request, game: &mut Game) -> Result<Self::Response, ApiError> {

		//TODO: check auth from basic credentials, make sure auth'ed team can access submarine
		let moved_time = game.move_submarine(request.submarine_id, (request.x, request.y))?;

		Ok(Response {
			move_cooldown: (moved_time + Submarine::move_cooldown()).get_millis(),
		})
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
	submarine_id: String,
	x: i32,
	y: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
	move_cooldown: u64
}

impl Validate for Request {
	fn validate(&self) -> ValidationResult {
		Ok(())
	}
}