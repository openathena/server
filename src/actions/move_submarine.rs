use super::ActionDef;
use api::error_handlers::ApiError;
use game::Game;
use hex_grid::*;
use std::time::Duration;
use validate::{Validate, ValidationResult};
use task_scheduler::Scheduler;
use std::sync::{Arc, Mutex};
use events::Visibility;

pub struct Definition;

impl ActionDef for Definition {
    const NAME: &'static str = "MOVE_SUBMARINE";
    type Request = Request;
    type Response = Response;

    fn execute(&self, request: Self::Request, game: &mut Game) -> Result<Self::Response, ApiError> {

        //TODO: check auth from basic credentials, make sure auth'ed team can access submarine
        game.move_submarine(request.submarine_id, (request.x, request.y))?;

        Ok(Response {})
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
pub struct Response {}

impl Validate for Request {
    fn validate(&self) -> ValidationResult {
        Ok(())
    }
}