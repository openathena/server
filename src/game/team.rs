use rand::{thread_rng, Rng};

use api::model::TeamInfo;

pub struct Team{
	id: String,
	name: String,
	password: String
}

impl Team{
	pub fn new(name: &str, password: &str) -> Team{
		Team{
			id: Self::generate_id(),
			name: name.to_owned(),
			password: password.to_owned()
		}
	}

	pub fn get_id(&self) -> String{
		self.id.clone()
	}

	pub fn get_name(&self) -> String{
		self.name.clone()
	}

	pub fn check_password(&self, password: &str) -> bool{
		self.password == password //TODO: timing attack?
	}

	fn generate_id() -> String{
		thread_rng()
				.gen_ascii_chars()
				.take(24)
				.collect::<String>()
	}

	pub fn get_info(&self) -> TeamInfo{
		TeamInfo{
			id: self.get_id(),
			name: self.get_name()
		}
	}
}