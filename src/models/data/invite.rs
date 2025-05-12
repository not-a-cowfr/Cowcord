#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use super::application::Application;
use super::channel::Channel;
use super::directory::GuildScheduledEvent;
use super::guild::GuildMember;
use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Invite {
	pub code:                       String,
	/// https://docs.discord.food/resources/invite#invite-type
	pub r#type:                     u8,
	pub channel:                    Option<Channel>,
	pub guild_id:                   Snowflake,
	pub guild:                      InviteGuild,
	pub inviter:                    User,
	/// https://docs.discord.food/resources/invite#invite-flags
	pub flags:                      u64,
	/// https://docs.discord.food/resources/invite#invite-target-type
	pub target_type:                u8,
	pub target_user:                User,
	pub target_application:         Application,
	pub approximate_member_count:   u32,
	pub approximate_presence_count: u32,
	pub expires_at:                 Option<Timestamp>,
	pub stage_instance:             InviteStageInstance,
	pub guild_scheduled_event:      GuildScheduledEvent,
	pub new_member:                 bool,
	pub show_verification_form:     bool,
	pub is_nickname_changeable:     bool,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum InviteType {
		GUILD = 0,
		GROUP_DM = 1,
		FRIEND = 2,
		_ => Unknown(u8),
	}
}

bitflags! {
  pub struct InviteFlags: u64 {
	const IS_GUEST_INVITE = 1 << 0;
	const IS_VIEWED = 1 << 1;
	const IS_ENHANCED = 1 << 2;
		const IS_APPLICATION_BYPASS = 1 << 3;
  }
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum InviteTargetType {
		STREAM = 1,
		EMBEDDED_APPLICATION = 2,
		ROLE_SUBSCRIPTIONS = 3,
		CREATOR_PAGE = 4,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InviteMetadata {
	pub uses:       u32,
	pub max_uses:   u32,
	pub max_age:    u32,
	pub temporary:  bool,
	pub created_at: Timestamp,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InviteGuild {
	pub id:                         Snowflake,
	pub name:                       String,
	pub icon:                       Option<String>,
	pub description:                Option<String>,
	pub banner:                     Option<String>,
	pub splash:                     Option<String>,
	/// https://docs.discord.food/resources/guild#verification-level
	pub verification_level:         u8,
	/// https://docs.discord.food/resources/guild#guild-features
	pub features:                   Vec<String>,
	pub vanity_url_code:            Option<String>,
	pub premium_subscription_count: u32,
	#[deprecated]
	pub nsfw:                       bool,
	/// https://docs.discord.food/resources/guild#nsfw-level
	pub nsfw_level:                 u8,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InviteStageInstance {
	pub members:           Vec<GuildMember>,
	pub participant_count: u16,
	pub speaker_count:     u16,
	pub topic:             String,
}
