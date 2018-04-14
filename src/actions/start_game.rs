use super::ActionDef;
use api::error_handlers::ApiError;
use game::Game;

pub struct Definition;

impl ActionDef for Definition {
	const NAME: &'static str = "START_GAME";
	type Request = ();
	type Response = Response;

	fn execute(&self, _request: Self::Request, game: &mut Game) -> Result<Self::Response, ApiError> {
		game.modify_state(|state| state.start())?;
		Ok(Response {})
	}
}

#[derive(Serialize)]
pub struct Response {}