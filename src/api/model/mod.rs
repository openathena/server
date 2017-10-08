#[derive(Deserialize)]
pub struct CreateTeamRequest {
	pub name: String,
	pub password: String
}

#[derive(Serialize)]
pub struct CreateTeamResponse {
	pub team_id: String
}