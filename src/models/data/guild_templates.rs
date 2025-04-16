#![allow(non_camel_case_types)]

use serde::Deserialize;

use super::guild::Guild;
use crate::models::types::{Snowflake, Timestamp};
use super::user::User;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GuildTemplate {
	pub code:                    String,
	pub name:                    String,
	pub description:             Option<String>,
	pub usage_count:             u32,
	pub creator_id:              Snowflake,
	pub creator:                 User,
	pub created_at:              Timestamp,
	pub updated_at:              Timestamp,
	pub source_guild_id:         Snowflake,
	pub serialized_source_guild: Guild,
	pub is_dirty:                Option<bool>,
}
