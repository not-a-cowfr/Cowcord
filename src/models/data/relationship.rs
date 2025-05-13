#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Relationship {
	pub id:                    String,
	/// https://docs.discord.food/resources/relationships#relationship-type
	pub r#type:                u8,
	pub user:                  User,
	pub nickname:              Option<String>,
	pub is_spam_request:       bool,
	pub stranger_request:      bool,
	pub user_ignored:          bool,
	pub origin_application_id: Option<Snowflake>,
	pub since:                 Timestamp,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RelationshipType {
	NONE = 0,
	FRIEND = 1,
	BLOCKED = 2,
	INCOMING_REQUEST = 3,
	OUTGOING_REQUEST = 4,
	IMPLICIT = 5,
	// SUGGESTION = 6,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GameRelationship {
	id:             String,
	application_id: Snowflake,
	r#type:         u8, // https://docs.discord.food/resources/relationships#game-relationship-type
	user:           User,
	since:          Timestamp,
	dm_access_type: u8, // https://docs.discord.food/resources/relationships#dm-access-type
	user_id:        Snowflake,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GameRelationshipType {
	FRIEND = 1,
	INCOMING_REQUEST = 3,
	OUTGOING_REQUEST = 4,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DmAccessType {
	// unknown
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FriendSuggestion {
	suggested_user:               User,
	reasons:                      Vec<FriendSuggestionReason>,
	from_suggested_user_contacts: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FriendSuggestionReason {
	r#type:   u8, // https://docs.discord.food/resources/relationships#friend-suggestion-reason-type
	platform: String,
	name:     String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FriendSuggestionType {
	EXTERNAL_FRIEND = 1,
}
