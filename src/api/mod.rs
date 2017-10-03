use rocket::{State, Route};
use rocket_contrib::Json;
use std::sync::{Mutex, Arc};

pub mod model;

use self::model::*;
use game::{Game, Team};

type GameMutex = Arc<Mutex<Game>>;

#[post("/teams", data = "<input>")]
fn create_team(input: Json<CreateTeamRequest>, game: State<GameMutex>) -> Json<TeamInfo> {
	let team = Team::new(&input.name, &input.password);
	let info = team.get_info();
	game.lock().unwrap().add_team(team);
	Json(info)
}

#[get("/teams")]
fn get_teams(game: State<GameMutex>) -> Json<Vec<TeamInfo>>{
	let game = game.lock().unwrap();
	Json(game.get_teams().iter().map(|t|t.get_info()).collect())
}

#[get("/teams/<team_id>")]
fn get_team(team_id: String, game: State<GameMutex>) -> Option<Json<TeamInfo>>{
	let game = game.lock().unwrap();
	game.get_team(&team_id).map(|t|Json(t.get_info()))
}

pub fn get_routes() -> Vec<Route> {
	routes![
		create_team, get_teams, get_team
	]
}