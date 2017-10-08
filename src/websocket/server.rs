use ws::{self, Sender};
use game::Game;
use std::sync::{Arc, Mutex};
use serde_json;
use super::request::{Request, AuthRequest};
use super::message::Message;
use game::events::{Event, Visibility, VisibleEvent};

const PORT: u16 = 43202;


pub struct Server {
	game: Arc<Mutex<Game>>
}

impl Server {
	pub fn new(game: Arc<Mutex<Game>>) -> Server {
		Server { game }
	}

	pub fn start(self) {
		ws::listen(("0.0.0.0", PORT), |sender: Sender| {
			println!("New Connection established");
			Handler::new(self.game.clone(), sender)
		}).unwrap();
	}
}

#[derive(Clone)]
enum AuthType {
	Observer,
	Team(String)
}

struct Handler {
	game: Arc<Mutex<Game>>,
	auth_type: Option<AuthType>,
	sender: Sender
}

impl Handler {
	pub fn new(game: Arc<Mutex<Game>>, sender: Sender) -> Handler {
		Handler {
			game,
			auth_type: None,
			sender
		}
	}
	fn authenticate_team(&mut self, auth_request: AuthRequest) -> Option<String> {
		if let Some(team) = self.game.lock().unwrap().get_team(&auth_request.team_id) {
			if team.check_password(&auth_request.password) {
				return Some(team.get_id());
			}
		}
		return None;
	}

	//	fn event_auth_filter<F>(auth_type: AuthType, listener: F)
	//		where F: FnMut(VisibleEvent) + Send + 'static {
	//
	//	}

	fn is_visible(auth_type: &AuthType, visibility: &Visibility) -> bool {
		true // only visibility is 'public' right now
	}

	fn filter_visible_events<F: FnMut(Event)>(auth_type: AuthType, mut listener: F) -> impl FnMut(VisibleEvent) {
		move |visible_event: VisibleEvent| {
			if Self::is_visible(&auth_type, &visible_event.get_visibility()) {
				listener(visible_event.get_event());
			}
		}
	}

	fn handle_request(&mut self, request: Request) -> ws::Result<()> {
		if self.auth_type.is_some() {
			return Err(ws::Error::new(ws::ErrorKind::Protocol, "Already Authenticated"));
		}
		match request {
			Request::Auth(auth_request) => {
				match self.authenticate_team(auth_request) {
					Some(team_id) => {
						self.auth_type = Some(AuthType::Team(team_id));
						self.setup_event_stream();
						Ok(())
					}
					None => Err(ws::Error::new(ws::ErrorKind::Protocol, "Authentication Failed"))
				}
			}
			Request::Observe => {
				self.auth_type = Some(AuthType::Observer);
				self.setup_event_stream();

				//				self.game.lock().unwrap().add_event_listener(move |visible_event| {
				//					if visible_event.get_visibility() == Visibility::Public {
				//						let message = serde_json::to_string(&Message::Event(visible_event.get_event())).unwrap();
				//						sender.send(ws::Message::text(message));
				//					}
				//				});

				Ok(())
			}
		}
	}
	fn setup_event_stream(&mut self) {
		if let Some(ref auth_type) = self.auth_type {
			let sender = self.sender.clone();

			{
				let mut game = self.game.lock().unwrap();

				//send all existing events as "History" messages
				game.get_event_history().iter().filter(|visible_event| {
					Self::is_visible(auth_type, &visible_event.get_visibility())
				}).for_each(|visible_event| {
					let message = Message::Event(visible_event.get_event());
					let history = Message::History(vec!(message));//TODO: batch up history events
					sender.send(ws::Message::text(serde_json::to_string(&history).unwrap()));//TODO: handle errors here
				});

				//listen for all future events
				game.add_event_listener(Self::filter_visible_events(auth_type.clone(), move |event| {
					let message = Message::Event(event);
					sender.send(ws::Message::text(serde_json::to_string(&message).unwrap()));//TODO: handle errors here
				}))
			}
		};
	}
}

impl ws::Handler for Handler {
	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		let request: Request = match serde_json::from_str(msg.as_text()?) {
			Ok(x) => x,
			Err(_) => return Err(ws::Error::new(ws::ErrorKind::Protocol, "Invalid Request"))
		};
		self.handle_request(request)
	}
}