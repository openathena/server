use hex_grid::*;
use game::Game;
use game::data::GameData;
use api::error_handlers::{ApiError, ApiErrorType};
use std::collections::HashMap;
use std::time::Duration;
use game::server_time::ServerTime;

#[derive(Clone)]
pub struct Submarine {
    id: String,
    coords: Coordinate,
    team_id: String,
    move_cooldown_end: ServerTime,
}

impl Submarine {
    pub fn new<T: Into<Coordinate>>(coords: T, team_id: &str) -> Submarine {
        Submarine {
            id: GameData::generate_id(),
            coords: coords.into(),
            team_id: team_id.to_owned(),
            move_cooldown_end: ServerTime::zero(),
        }
    }

    pub fn move_cooldown() -> Duration {
        Duration::from_secs(10)
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_coords(&self) -> Coordinate {
        self.coords
    }

    pub fn get_team_id(&self) -> String {
        self.team_id.to_owned()
    }

    pub fn get_move_cooldown_end(&self) -> ServerTime {
        self.move_cooldown_end.clone()
    }

    pub fn move_to(&mut self, dest: Coordinate, cooldown_time: ServerTime) {
        self.coords = dest;
        self.move_cooldown_end = cooldown_time;
    }
}