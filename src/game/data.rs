use game::team::Team;
use std::collections::HashMap;
use hex_grid::*;
use events::{VisibleEvent, Visibility};
use game::submarine::Submarine;
use events::{Event, EventType};
use std::time::Instant;
use api::error_handlers::{ApiError, ApiErrorType};
use game::server_time::ServerTime;
use events::*;
use game::auth::*;
use rand::{thread_rng, Rng};

pub struct GameData {
    pub game_start: Instant,
    pub state: State,
    pub teams: HashMap<String, Team>,
    pub world: HashMap<Coordinate, TileType>,
    pub event_history: Vec<VisibleEvent>,
    pub event_listeners: HashMap<String, Box<FnMut(VisibleEvent) + Send>>,
    pub submarines: HashMap<String, Submarine>,
}

impl GameData {
    pub fn server_time(&self) -> ServerTime {
        ServerTime::new(self.game_start.elapsed())
    }
    pub fn generate_event<T: EventType>(&mut self, visibility: Visibility, event_data: &T) -> Result<ServerTime, ApiError> {
        let server_time = self.server_time();
        self.generate_timestamped_event(server_time, visibility, event_data)
    }

    pub fn generate_timestamped_event<T: EventType>(&mut self, time: ServerTime, visibility: Visibility, event_data: &T) -> Result<ServerTime, ApiError> {
        let visible_event = VisibleEvent::new(visibility, Event::new(event_data, time)?);
        for listener in &mut self.event_listeners.values_mut() {
            listener(visible_event.clone())
        }
        self.event_history.push(visible_event);
        Ok(time)
    }

    pub fn add_team(&mut self, team: Team) -> Result<(), ApiError> {
        self.state.require_state(State::TeamCreation)?;
        self.teams.insert(team.get_id().to_owned(), team.clone());
        self.generate_event(Visibility::Public, &TeamCreated {
            id: team.get_id(),
            name: team.get_name(),
        })?;
        Ok(())
    }
    pub fn get_team(&self, id: &str) -> Option<&Team> {
        self.teams.get(id)
    }
    pub fn add_event_listener<F>(&mut self, listener: F) -> String
        where F: FnMut(VisibleEvent) + Send + 'static {
        let id = Self::generate_id();
        self.event_listeners.insert(id.clone(), Box::new(listener));
        id
    }
    pub fn remove_event_listener(&mut self, id: &str) {
        self.event_listeners.remove(id);
    }
    pub fn auth(&self, credentials: Option<(&str, &str)>) -> Result<AuthType, ApiError> {
        if let Some((username, password)) = credentials {
            if let Some(team) = self.get_team(username) {
                if team.check_password(password) {
                    Ok(AuthType::Team(team.get_id()))
                } else {
                    Err(ApiError::new(ApiErrorType::Unauthorized, "Invalid username or password"))
                }
            } else {
                Err(ApiError::new(ApiErrorType::Unauthorized, "Invalid username or password"))
            }
        } else {
            Ok(AuthType::Observer)
        }
    }

    pub fn move_submarine<C: Into<Coordinate>>(&mut self, sub_id: String, destination: C) -> Result<ServerTime, ApiError> {
        let destination = destination.into();
        let move_time = self.server_time();
        let (visibility, event) = {
            let sub = match self.submarines.get_mut(&sub_id) {
                Some(sub) => sub,
                None => return Err(ApiError::new(ApiErrorType::BadRequest, "Invalid submarine id"))
            };
            if (destination - sub.get_coords()).distance() != 1 {
                return Err(ApiError::new(ApiErrorType::BadRequest, "Submarine must move exactly 1 tile"));
            }
            if !self.world.contains_key(&destination) {
                return Err(ApiError::new(ApiErrorType::BadRequest, "Invalid destination"));
            }

            if sub.get_move_cooldown_end() <= move_time {
                return Err(ApiError::new(ApiErrorType::BadRequest, "Move not available"));
            }
            sub.move_to(destination, move_time + Submarine::move_cooldown());

            (
                Visibility::Team(sub.get_team_id()),
                &SubmarineMoved {
                    x: sub.get_coords().x,
                    y: sub.get_coords().y,
                    submarine_id: sub.get_id(),
                }
            )
        };

        self.generate_timestamped_event(move_time, visibility, event)
    }

    pub fn start(&mut self) -> Result<(), ApiError> {
        self.state.require_state(State::TeamCreation)?;
        self.generate_world()?;
        self.place_starting_submarines()?;
        self.generate_event(Visibility::Public, &GameStarted)?;
        self.state = State::Started;
        Ok(())
    }

    fn generate_world(&mut self) -> Result<(), ApiError> {
        let default_tile = TileType::Water;

        //TODO: not sure what the size/shape should be yet
        //TODO: make sure map is large enough to fit ships for all teams

        let coords = CENTER + Offset::fill_hex(8);
        for coord in coords {
            self.world.insert(coord, default_tile);
            self.generate_event(Visibility::Public, &TileUpdated {
                x: coord.x,
                y: coord.y,
                tile: tile_updated::Tile { tile_type: default_tile },
            })?;
        }
        Ok(())
    }

    fn place_starting_submarines(&mut self) -> Result<(), ApiError> {
        let mut empty_tiles: Vec<Coordinate> = self.world.keys()
            .map(|x| x.clone())
            .filter(|x| self.is_tile_empty(*x))
            .collect();
        thread_rng().shuffle(&mut empty_tiles);
        let teams: Vec<Team> = self.teams.values().map(|x| x.clone()).collect();
        for team in teams {
            let coord = if let Some(coord) = empty_tiles.pop() {
                coord
            } else {
                return Err(ApiError::new(ApiErrorType::InternalServerError, "Not enough tiles for ships"));
            };
            let submarine = Submarine::new(coord, &team.get_id());
            self.submarines.insert(submarine.get_id(), submarine.clone());
            self.generate_event(Visibility::Team(team.get_id()), &SubmarineCreated {
                x: coord.x,
                y: coord.y,
                id: submarine.get_id(),
                team_id: submarine.get_team_id(),
            })?;
        }
        Ok(())
    }

    pub fn is_tile_empty<T: Into<Coordinate>>(&self, coord: T) -> bool {
        self.get_submarine_at(coord).is_none()
    }

    pub fn get_submarine_at<T: Into<Coordinate>>(&self, coords: T) -> Option<&Submarine> {
        let coords = coords.into();
        self.submarines.values().find(move |x| x.get_coords() == coords)
    }

    pub fn generate_id() -> String {
        thread_rng()
            .gen_ascii_chars()
            .take(24)
            .collect::<String>()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    TeamCreation,
    Started,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TileType {
    Water
}