#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::guild::Guild;
use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FamilyCenter {
	pub linked_users:   Vec<LinkedUser>,
	pub teen_audit_log: TeenAuditLog,
	pub users:          Vec<User>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LinkStatus {
	SENT = 1,
	CONNECTED = 2,
	DISCONNECTED = 3,
	REJECTED = 4,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LinkType {
	RECIEVER = 1,
	SENDER = 2,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinkedUser {
	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	pub link_status:  LinkStatus,
	pub link_type:    LinkType,
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
	pub totals:         HashMap<ActionType, u16>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Action {
	pub event_id:     Snowflake,
	pub user_id:      Snowflake,
	pub entity_id:    Snowflake,
	pub display_type: ActionType,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionType {
	USERS_ADDED = 1,
	GUILDS_JOINED = 2,
	USERS_MESSAGED = 3,
	GUILDS_MESSAGED = 4,
	USERS_CALLED = 5,
}
