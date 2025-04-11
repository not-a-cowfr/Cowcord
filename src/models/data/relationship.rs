#![allow(non_camel_case_types)]

use serde::Deserialize;

use super::types::{Snowflake, Timestamp};
use super::user::User;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Relationship {
	pub id:                    String,
	pub r#type:                u8, // https://docs.discord.sex/resources/relationships#relationship-type
	pub user:                  User,
	pub nickname:              Option<String>,
	pub is_spam_request:       bool,
	pub stranger_request:      bool,
	pub user_ignored:          bool,
	pub origin_application_id: Option<Snowflake>,
	pub since:                 Timestamp,
}

pub enum RelationshipType {
	NONE = 0,
	FRIEND = 1,
	BLOCKED = 2,
	INCOMING_REQUEST = 3,
	OUTGOING_REQUEST = 4,
	IMPLICIT = 5,
	// SUGGESTION = 6,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GameRelationship {
	id:             String,
	application_id: Snowflake,
	r#type:         u8, // https://docs.discord.sex/resources/relationships#game-relationship-type
	user:           User,
	since:          Timestamp,
	dm_access_type: u8, // https://docs.discord.sex/resources/relationships#dm-access-type
	user_id:        Snowflake,
}

pub enum GameRelationshipType {
	// unknown
}

pub enum DmAccessType {
	// unknown
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct FriendSuggestion {
	suggested_user:               User,
	reasons:                      Vec<FriendSuggestionReason>,
	from_suggested_user_contacts: bool,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct FriendSuggestionReason {
	r#type:   u8, // https://docs.discord.sex/resources/relationships#friend-suggestion-reason-type
	platform: String,
	name:     String,
}

pub enum FriendSuggestionType {
	EXTERNAL_FRIEND = 1,
}
