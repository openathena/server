use ws::{self, Sender};
use game::Game;
use serde_json;
use super::request::Request;
use super::message::Message;
use api::error_handlers::{ApiErrorType, ApiError};
use game::auth::AuthType;
use events::VisibleEvent;

const PORT: u16 = 43202;


pub struct Server {
	game: Game
}

impl Server {
	pub fn new(game: Game) -> Server {
		Server { game }
	}

	pub fn start(self) {
		ws::listen(("0.0.0.0", PORT), |sender: Sender| {
			println!("New Connection established");
			Handler::new(self.game.clone(), sender)
		}).unwrap();
	}
}

struct Handler {
	game: Game,
	auth_type: AuthType,
	sender: Sender,
	listener_id: Option<String>,
}

impl Handler {
	pub fn new(game: Game, sender: Sender) -> Handler {
		Handler {
			game,
			auth_type: AuthType::Observer,
			sender,
			listener_id: None,
		}
	}

	fn shutdown_event_stream(&mut self) -> bool {
		if let Some(team_id) = self.listener_id.take() {
			self.game.modify_state(|state| {
				state.remove_event_listener(&team_id);
			});
			true
		} else {
			false
		}
	}

	fn handle_request(&mut self, msg: &str) -> Result<(), ApiError> {
		let request: Request = serde_json::from_str(msg).map_err(|_| {
			ApiError::new(ApiErrorType::BadRequest, "Invalid Request")
		})?;
		if self.shutdown_event_stream() {
			Self::send_message(&self.sender, &Message::Reset)
		}
		match request {
			Request::Auth(auth_request) => {
				let credentials = Some((auth_request.username.as_ref(), auth_request.password.as_ref()));
				self.auth_type = self.game.modify_state(|state| state.auth(credentials))?;
			}
			Request::Observe => {
				self.auth_type = self.game.modify_state(|state| state.auth(None))?;
			}
		}
		self.setup_event_stream();
		Ok(())
	}

	fn send_message(sender: &ws::Sender, msg: &Message) {
		let _err = sender.send(ws::Message::text(serde_json::to_string(&msg).unwrap()));
	}

	fn setup_event_stream(&mut self) {
		let sender = self.sender.clone();
		//send all existing events as "History" messages

		self.game.iter_event_history(|visible_event: &VisibleEvent| {
			if self.auth_type.require_visibility(&visible_event.get_visibility()).is_err() {
				return;
			}
			let message = Message::Event(visible_event.get_event());
			let history = Message::History(vec!(message));//TODO: batch up history messages
			Self::send_message(&sender, &history);
		});

		let empty_history = Message::History(vec!());//this marks the end of history messages
		Self::send_message(&sender, &empty_history);

		//listen for all future events
		let auth_type = self.auth_type.clone();
		self.listener_id = Some(self.game.modify_state(|state| {
			state.add_event_listener(move |event| {
				if auth_type.require_visibility(&event.get_visibility()).is_ok() {
					let message = Message::Event(event.get_event());
					Self::send_message(&sender, &message);
				}
			})
		}));
	}
}

impl ws::Handler for Handler {
	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		let msg_text = msg.as_text()?;
		if let Err(api_error) = self.handle_request(msg_text) {
			let err_string = serde_json::to_string(&api_error).unwrap();
			return self.sender.close_with_reason(ws::CloseCode::Policy, err_string);
		}
		Ok(())
	}
	fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
		self.shutdown_event_stream();
	}
}