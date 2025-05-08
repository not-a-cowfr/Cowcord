use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::guild::Guild;
use super::user::User;
use crate::enum_number;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FamilyCenter {
	pub linked_users:   Vec<LinkedUser>,
	pub teen_audit_log: TeenAuditLog,
	pub users:          Vec<User>,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum LinkStatus {
		SENT = 1,
		CONNECTED = 2,
		DISCONNECTED = 3,
		REJECTED = 4,
		_ => Unknown(u8),
	}
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum LinkType {
		RECIEVER = 1,
		SENDER = 2,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinkedUser {
	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	/// https://docs.discord.food/resources/family-center#link-status
	pub link_status:  u8,
	/// https://docs.discord.food/resources/family-center#link-type
	pub link_type:    u8,
	pub requestor_id: String,
	pub user_id:      String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinkedUsers {
	pub linked_users: Vec<LinkedUser>,
	pub users:        Vec<User>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TeenAuditLog {
	pub teen_user_id:   Option<Snowflake>,
	pub range_start_id: Option<Snowflake>,
	pub actions:        Vec<Action>,
	pub users:          Vec<User>,
	pub guilds:         Vec<Guild>,
	pub totals:         HashMap<u8, u16>, /* the type (first one) _might_ be returned as a string not an int but maybe serde will dela with that for me */
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Action {
	pub event_id:     Snowflake,
	pub user_id:      Snowflake,
	pub entity_id:    Snowflake,
	/// https://docs.discord.food/resources/family-center#action-type
	pub display_type: u8,
}
