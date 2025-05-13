use serde::{Deserialize, Serialize};

use super::guild::{GuildFeatures, PremiumTier};
use super::user::User;
use crate::models::types::Snowflake;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Emoji {
	pub id:             Option<Snowflake>,
	pub name:           String,
	pub roles:          Option<Vec<Snowflake>>,
	pub user:           User,
	pub require_colons: bool,
	pub managed:        bool,
	pub animated:       bool,
	pub available:      bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TopEmoji {
	pub emoji_id:   Snowflake,
	pub emoji_rank: u16,
}

pub enum EmojiSourceType {
	GUILD,
	APPLICATION,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmojiGuild {
	pub id:                         Snowflake,
	pub name:                       String,
	pub icon:                       Option<String>,
	pub description:                Option<String>,
	pub features:                   Vec<GuildFeatures>,
	pub emojis:                     Vec<Emoji>,
	pub premium_tier:               PremiumTier,
	pub premium_subscription_count: u32,
	pub approximate_member_count:   u32,
	pub approximate_presence_count: u32,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmojiApplication {
	pub id:   Snowflake,
	pub name: String,
}
