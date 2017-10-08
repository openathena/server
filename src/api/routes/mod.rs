use rocket_contrib::Json;
use rocket::{Route, State};
use std::sync::{Arc, Mutex};
use game::{Game, Team};
use game::actions::Action;
use api::model::*;

type GameMutex = Arc<Mutex<Game>>;

#[post("/teams", data = "<request>")]
fn create_team(request: Json<CreateTeamRequest>, game: State<GameMutex>) -> Json<CreateTeamResponse> {
	let event = Team::create_event(&request.name, &request.password);
	let team_id = event.id.clone();
	game.lock().unwrap().execute_action(Action::CreateTeam(event));
	Json(CreateTeamResponse { team_id })
}

pub fn get_routes() -> Vec<Route> {
	routes![
		create_team
	]
}