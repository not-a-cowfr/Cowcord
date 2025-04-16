use serde::{Deserialize, Serialize};

use crate::models::types::Snowflake;
use super::user::User;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Emoji {
	pub id:             Option<Snowflake>,
	pub name:           String, // may be null if emoji has been deleted
	pub roles:          Vec<Snowflake>,
	pub user:           User,
	pub require_colons: bool,
	pub managed:        bool,
	pub animated:       bool,
	pub available:      bool,
}
