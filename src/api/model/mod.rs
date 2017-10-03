#[derive(Deserialize)]
pub struct CreateTeamRequest{
	pub name: String,
	pub password: String
}

#[derive(Serialize)]
pub struct TeamInfo{
	pub id: String,
	pub name: String
}