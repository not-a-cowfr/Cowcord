use iso8601_timestamp::Timestamp;
use serde::Deserialize;

use super::types::Snowflake;
use super::user::{AvatarDecorationData, User};

#[derive(Deserialize, Debug)]
pub struct GuildMember {
	pub user:                         User,
	pub nick:                         Option<String>,
	pub avatar:                       Option<String>,
	pub avatar_decoration_data:       Option<AvatarDecorationData>,
	pub banner:                       Option<String>,
	pub roles:                        Vec<Snowflake>,
	pub joined_at:                    Timestamp,
	pub premium_since:                Option<Timestamp>,
	pub deaf:                         bool,
	pub mute:                         bool,
	pub pending:                      bool,
	pub communication_disabled_until: Option<Timestamp>,
	pub unusual_dm_activity_until:    Option<Timestamp>,
	pub flags:                        u8, // https://docs.discord.sex/resources/guild#guild-member-flags
	pub permissions:                  String,
}
