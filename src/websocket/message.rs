
use game::events::Event;

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Message{
	Event(Event),
	History(Vec<Message>)
}