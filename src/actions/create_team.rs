use super::ActionDef;
use validate::{Validate, ValidationResult};
use validate::rules::*;
use api::error_handlers::ApiError;
use game::Game;
use game::team::Team;

pub struct Definition;

impl ActionDef for Definition {
	const NAME: &'static str = "CREATE_TEAM";
	type Request = Request;
	type Response = Response;

	fn execute(&self, request: Self::Request, game: &mut Game) -> Result<Response, ApiError> {
		let team = Team::new(&request.name, &request.password);
		let team_id = team.get_id();
		game.add_team(team);
		Ok(Response { team_id })
	}
}

#[derive(Deserialize)]
pub struct Request {
	pub name: String,
	pub password: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
	pub team_id: String
}

impl Validate for Request {
	fn validate(&self) -> ValidationResult {
		bound(..21).name("name length").validate(&self.name.len())?;
		bound(..51).name("password length").validate(&self.password.len())
	}
}