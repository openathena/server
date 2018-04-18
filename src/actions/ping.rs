use super::ActionDef;
use api::error_handlers::ApiError;
use game::Game;

pub struct Definition;

impl ActionDef for Definition {
    const NAME: &'static str = "PING";
    type Request = ();
    type Response = Response;

    fn execute(&self, _request: Self::Request, game: &mut Game) -> Result<Self::Response, ApiError> {
        Ok(Response {
            server_time: game.modify_state(|state|state.server_time().get_millis()),
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    server_time: u64
}
