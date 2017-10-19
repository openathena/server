use api::error_handlers::{ApiErrorType, ApiError};
use events::Visibility;

#[derive(Clone)]
pub enum AuthType {
	Observer,
	Team(String)
}

impl AuthType {
	pub fn require_team(&self, id: &str) -> Result<(), ApiError> {
		match *self {
			AuthType::Observer => Self::auth_error(),
			AuthType::Team(ref team_id) => {
				if team_id == id {
					Ok(())
				} else {
					Self::auth_error()
				}
			}
		}
	}

	pub fn require_visibility(&self, visibility: &Visibility) -> Result<(), ApiError> {
		match *visibility {
			Visibility::Public => Ok(()),
			Visibility::Team(ref team_id) => self.require_team(team_id)
		}
	}

	fn auth_error() -> Result<(), ApiError> {
		Err(ApiError::new(ApiErrorType::Forbidden, "Access Denied"))
	}
}