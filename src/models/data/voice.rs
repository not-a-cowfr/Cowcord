use serde::Deserialize;

use super::guild::GuildMember;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VoiceState {
	pub guild_id:                   Option<Snowflake>,
	pub channel_id:                 Option<Snowflake>,
	pub user_id:                    Snowflake,
	pub member:                     GuildMember,
	pub session_id:                 String,
	pub deaf:                       bool,
	pub mute:                       bool,
	pub self_deaf:                  bool,
	pub self_mute:                  bool,
	pub self_stream:                bool,
	pub self_video:                 bool,
	pub suppress:                   bool,
	pub request_to_speak_timestamp: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VoiceRegion {
	pub id:         String,
	pub name:       String,
	pub optimal:    bool,
	pub deprecated: bool,
	pub custom:     bool,
}
