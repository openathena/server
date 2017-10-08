pub mod create_team;

pub use self::create_team::CreateTeam;

use game::Game;

#[derive(Clone)]
pub enum Action {
	CreateTeam(CreateTeam)
}