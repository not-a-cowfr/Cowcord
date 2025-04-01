use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use super::guild::GuildMember;

const DISCORD_EPOCH: u64 = 1420070400000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Snowflake(u64);

impl Snowflake {
	pub fn new(id: u64) -> Self { Snowflake(id) }

	pub fn generate(
		worker_id: u8,
		process_id: u8,
		increment: &AtomicU16,
	) -> Self {
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_millis() as u64
			- DISCORD_EPOCH;

		let worker_id = worker_id & 0b11111;
		let process_id = process_id & 0b11111;

		let incr = increment.fetch_add(1, Ordering::SeqCst) & 0b111111111111;

		let id = (timestamp << 22)
			| ((worker_id as u64) << 17)
			| ((process_id as u64) << 12)
			| (incr as u64);
		Snowflake(id)
	}

	pub fn timestamp(&self) -> u64 { (self.0 >> 22) + DISCORD_EPOCH }

	pub fn worker_id(&self) -> u8 { ((self.0 >> 17) & 0b11111) as u8 }

	pub fn process_id(&self) -> u8 { ((self.0 >> 12) & 0b11111) as u8 }

	pub fn increment(&self) -> u16 { (self.0 & 0b111111111111) as u16 }

	pub fn raw(&self) -> u64 { self.0 }
}

impl From<Snowflake> for u64 {
	fn from(snowflake: Snowflake) -> Self { snowflake.0 }
}

impl From<u64> for Snowflake {
	fn from(value: u64) -> Self { Snowflake(value) }
}

impl std::fmt::Display for Snowflake {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

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
struct LinkedUser {
	created_at:   Timestamp,
	updated_at:   Timestamp,
	link_status:  u8, // https://docs.discord.sex/resources/family-center#link-status
	link_type:    u8, // https://docs.discord.sex/resources/family-center#link-type
	requestor_id: String,
	user_id:      String,
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
