use std::ops::Add;
use std::time::Duration;

const MILLIS_PER_SEC: u64 = 1000;
const NANOS_PER_MILLI: u32 = 1000_000;

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct ServerTime {
	duration: Duration
}

impl ServerTime {
	pub fn new(duration: Duration) -> ServerTime {
		ServerTime { duration }
	}
	pub fn get_millis(&self) -> u64 {
		(self.duration.as_secs() * MILLIS_PER_SEC) + (self.duration.subsec_nanos() / NANOS_PER_MILLI) as u64
	}
	pub fn zero() -> ServerTime {
		ServerTime::new(Duration::from_secs(0))
	}
}

impl Add<Duration> for ServerTime {
	type Output = ServerTime;

	fn add(self, duration: Duration) -> ServerTime {
		ServerTime {
			duration: self.duration + duration
		}
	}
}
