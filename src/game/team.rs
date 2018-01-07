use game::Game;
use game::data::GameData;

#[derive(Clone)]
pub struct Team {
    id: String,
    name: String,
    password: String,
}

impl Team {
    pub fn new(name: &str, password: &str) -> Team {
        Team {
            id: GameData::generate_id(),
            name: name.to_owned(),
            password: password.to_owned(),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn check_password(&self, password: &str) -> bool {
        self.password == password //TODO: fix timing attack
    }
}