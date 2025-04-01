use iso8601_timestamp::Timestamp;
use serde::Deserialize;

use super::guild::GuildMember;
use super::types::Snowflake;

#[derive(Deserialize, Debug)]
pub struct User {
	pub id:                      Snowflake,
	pub username:                String,
	pub global_name:             Option<String>,
	pub avatar:                  Option<String>,
	pub avatar_decoration_data:  Option<AvatarDecorationData>,
	pub primary_guild:           Option<PrimaryGuild>,
	pub linked_users:            Vec<LinkedUser>,
	pub bot:                     bool,
	pub system:                  bool,
	pub mfa_enabled:             bool,
	pub nsfw_allowed:            Option<bool>,
	pub age_verification_status: u8, // https://docs.discord.sex/resources/user#age-verification-status
	pub pronouns:                String,
	pub bio:                     String,
	pub banner:                  Option<String>,
	pub accent_color:            Option<u32>,
	pub locale:                  String, // https://docs.discord.sex/reference#locales
	pub verified:                bool,
	pub email:                   Option<String>,
	pub phone:                   Option<String>,
	pub premium_type:            u8, // https://docs.discord.sex/resources/user#premium-type
	pub personal_connection_id:  Snowflake,
	pub flags:                   u64, // https://docs.discord.sex/resources/user#user-flags
	pub public_flags:            u64,
	pub purchased_flags:         u8, // https://docs.discord.sex/resources/user#purchased-flags
	pub premium_flags:           u8, // https://docs.discord.sex/resources/user#premium-usage-flags
	pub desktop:                 bool,
	pub mobile:                  bool,
	pub has_bounced_email:       bool,
	pub authenticator_types:     Vec<u8>, // https://docs.discord.sex/resources/user#authenticator-type
}

#[derive(Deserialize, Debug)]
pub struct AvatarDecorationData {
	pub asset:      String,
	pub sku_id:     String,
	pub expires_at: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct PrimaryGuild {
	pub identity_enabled:  Option<bool>,
	pub identity_guild_id: Option<String>,
	pub tag:               Option<String>,
	pub badge:             Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LinkedUser {
	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	pub link_status:  u8, // https://docs.discord.sex/resources/family-center#link-status
	pub link_type:    u8, // https://docs.discord.sex/resources/family-center#link-type
	pub requestor_id: String,
	pub user_id:      String,
}

#[derive(Deserialize, Debug)]
pub struct Nick {
	pub id:   Snowflake,
	pub nick: String,
}

#[derive(Deserialize, Debug)]
pub struct ThreadMember {
	pub id:             Snowflake,
	pub user_id:        Snowflake,
	pub join_timestamp: Timestamp,
	pub flags:          u8, // https://docs.discord.sex/resources/channel#thread-member-flags
	pub muted:          bool,
	pub mute_config:    MuteConfig,
	pub member:         GuildMember,
}

#[derive(Deserialize, Debug)]
pub struct MuteConfig {
	pub end_time:             Option<Timestamp>,
	pub selected_time_window: isize,
}
