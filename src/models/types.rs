use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const DISCORD_EPOCH: u64 = 1420070400000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Snowflake(u64);

impl Snowflake {
	pub fn new(id: u64) -> Self { Snowflake(id) }

	pub fn generate(
		worker_id: u8,
		process_id: u8,
		increment: &AtomicU16,
	) -> Self {
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_millis() as u64
			- DISCORD_EPOCH;

		let worker_id = worker_id & 0b11111;
		let process_id = process_id & 0b11111;

		let incr = increment.fetch_add(1, Ordering::SeqCst) & 0b111111111111;

		let id = (timestamp << 22)
			| ((worker_id as u64) << 17)
			| ((process_id as u64) << 12)
			| (incr as u64);
		Snowflake(id)
	}

	pub fn timestamp(&self) -> u64 { (self.0 >> 22) + DISCORD_EPOCH }

	pub fn worker_id(&self) -> u8 { ((self.0 >> 17) & 0b11111) as u8 }

	pub fn process_id(&self) -> u8 { ((self.0 >> 12) & 0b11111) as u8 }

	pub fn increment(&self) -> u16 { (self.0 & 0b111111111111) as u16 }

	pub fn raw(&self) -> u64 { self.0 }
}

impl From<Snowflake> for u64 {
	fn from(snowflake: Snowflake) -> Self { snowflake.0 }
}

impl From<u64> for Snowflake {
	fn from(value: u64) -> Self { Snowflake(value) }
}

impl std::fmt::Display for Snowflake {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
