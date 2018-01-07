pub mod auth;
pub mod team;
pub mod submarine;
pub mod server_time;
pub mod data;

use self::data::GameData;
use events::*;
pub use self::team::Team;
pub use self::auth::AuthType;
use api::error_handlers::{ApiError, ApiErrorType};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use hex_grid::*;
use self::submarine::Submarine;
use std::time::Instant;
use task_scheduler::Scheduler;
use self::server_time::ServerTime;
use std::sync::{Arc, Mutex};
use self::data::*;

impl State {
    pub fn require_state(&self, state: State) -> Result<(), ApiError> {
        if *self == state {
            Ok(())
        } else {
            let msg = &format!("Illegal game state for action. Required '{:?}' but is currently '{:?}'", state, *self);
            Err(ApiError::new(ApiErrorType::BadRequest, msg))
        }
    }
}

pub struct Game {
    scheduler: Scheduler,
    data: Arc<Mutex<GameData>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            scheduler: Scheduler::new(),
            data: Arc::new(Mutex::new(GameData {
                game_start: Instant::now(),
                state: State::TeamCreation,
                teams: HashMap::new(),
                world: HashMap::new(),
                event_history: Vec::new(),
                event_listeners: HashMap::new(),
                submarines: HashMap::new(),
            })),
        }
    }

    pub fn get_scheduler(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }


    pub fn add_team(&mut self, team: Team) -> Result<(), ApiError> {
        self.data.lock().unwrap().add_team(team)
    }

    pub fn auth(&self, credentials: Option<(&str, &str)>) -> Result<AuthType, ApiError> {
        self.data.lock().unwrap().auth(credentials)
    }

    pub fn add_event_listener<F>(&mut self, listener: F) -> String
        where F: FnMut(VisibleEvent) + Send + 'static {
        self.data.lock().unwrap().add_event_listener(listener)
    }

    pub fn remove_event_listener(&mut self, id: &str) {
        self.data.lock().unwrap().remove_event_listener(id)
    }

//    pub fn get_event_history(&self) -> &Vec<VisibleEvent> {
////        &self.event_history
//        //TODO: take a function to iterate over event history
//        &vec!()
//    }

    pub fn start(&mut self) -> Result<(), ApiError> {
        self.data.lock().unwrap().start()
    }

//    pub fn get_submarine(&self, sub_id: &str) -> Result<&Submarine, ApiError> {
//        match self.submarines.get(&sub_id.to_owned()) {
//            Some(sub) => Ok(sub),
//            None => Err(ApiError::new(ApiErrorType::BadRequest, "Invalid submarine id"))
//        }
//    }

    pub fn move_submarine<C: Into<Coordinate>>(&mut self, sub_id: String, destination: C) -> Result<(), ApiError> {
        let mut data = self.data.lock().unwrap();
        let moved_time = data.move_submarine(sub_id, destination)?;

        //TODO: schedule cooldown reset event
//        {
//            self.get_scheduler().af.after_duration(cooldown, || {
//                let mut game = game_mutex_clone.lock().unwrap();
//                game.reset_submarine_cooldown();
//                game.generate_event(Visibility::Team(team_id), );
//            });
//        }

        Ok(())
    }

//    pub fn reset_submarine_cooldown(&mut self, sub_id: String) -> Result<(), ApiError> {
//
//    }
}