use rand::{thread_rng, Rng};

use super::actions::CreateTeam;

pub struct Team {
	id: String,
	name: String,
	password: String
}

impl Team {
	pub fn new(info: CreateTeam) -> Team {
		Team {
			id: info.id,
			name: info.name,
			password: info.password
		}
	}

	pub fn create_event(name: &str, password: &str) -> CreateTeam {
		CreateTeam {
			id: Self::generate_id(),
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

	fn generate_id() -> String {
		thread_rng()
				.gen_ascii_chars()
				.take(24)
				.collect::<String>()
	}
}