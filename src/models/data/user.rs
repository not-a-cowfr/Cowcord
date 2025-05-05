#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::emoji::Emoji;
use super::family_center::LinkedUser;
use super::integration::{IntegrationAccount, IntegrationGuild};
use crate::bitflags;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct User {
	pub id:                      Snowflake,
	pub username:                String,
	#[deprecated]
	pub discriminator:           String,
	pub global_name:             Option<String>,
	pub avatar:                  Option<String>,
	pub avatar_decoration_data:  Option<AvatarDecorationData>,
	pub primary_guild:           Option<PrimaryGuild>,
	pub linked_users:            Vec<LinkedUser>,
	pub bot:                     bool,
	pub system:                  bool,
	pub mfa_enabled:             bool,
	pub nsfw_allowed:            Option<bool>,
	/// https://docs.discord.sex/resources/user#age-verification-status
	pub age_verification_status: u8,
	pub pronouns:                String,
	pub bio:                     String,
	pub banner:                  Option<String>,
	pub accent_color:            Option<u32>,
	/// https://docs.discord.sex/reference#locales
	pub locale:                  String,
	pub verified:                bool,
	pub email:                   Option<String>,
	pub phone:                   Option<String>,
	/// https://docs.discord.sex/resources/user#premium-type
	pub premium_type:            u8,
	pub personal_connection_id:  Snowflake,
	/// https://docs.discord.sex/resources/user#user-flags
	pub flags:                   u64,
	pub public_flags:            u64,
	/// https://docs.discord.sex/resources/user#purchased-flags
	pub purchased_flags:         u64,
	/// https://docs.discord.sex/resources/user#premium-usage-flags
	pub premium_flags:           u64,
	pub desktop:                 bool,
	pub mobile:                  bool,
	pub has_bounced_email:       bool,
	/// https://docs.discord.sex/resources/user#authenticator-type
	pub authenticator_types:     Vec<u8>,
}

// not doing all this bro https://docs.discord.sex/resources/user#user-flags

bitflags! {
  pub struct PurchasedFlags: u64 {
	const NITRO_CLASSIC = 1 << 0;
	const NITRO = 1 << 1;
	const GUILD_BOOST = 1 << 2;
		const NITRO_BASIC = 1 << 3;
  }
}

bitflags! {
  pub struct PremiumFlags: u64 {
	const PREMIUM_DISCRIMINATOR = 1 << 0;
	const ANIMATED_AVATAR = 1 << 1;
		const PROFILE_BANNER = 1 << 2;
  }
}

pub enum PremuimType {
	NONE, // deprecated
	TIER_1,
	TIER_2,
	TIER_3,
}

pub enum AgeVerificationStatus {
	UNVERIFIED,
	VERIFIED_TEEN,
	VERIFIED_ADULT,
}

pub enum RequiredActionType {
	UPDATE_AGREEMENTS,
	// COMPLETE_CAPTCHA,
	VERIFY_EMAIL,
	REVERIFY_EMAIL,
	VERIFY_PHONE,
	REVERIFY_PHONE,
	// VERIFY_PHONE_THEN_EMAIL,
	VERIFY_EMAIL_OR_VERIFY_PHONE,
	REVERIFY_EMAIL_OR_VERIFY_PHONE,
	VERIFY_EMAIL_OR_REVERIFY_PHONE,
	REVERIFY_EMAIL_OR_REVERIFY_PHONE,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AvatarDecorationData {
	pub asset:      String,
	pub sku_id:     String,
	pub expires_at: Option<usize>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Collectibles {
	pub nameplate: NameplateData,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NameplateData {
	pub asset:      String,
	pub sku_id:     Snowflake,
	pub label:      String,
	pub palette:    String, // https://discord-userdoccers-1ifxqpgzv-discord-userdoccers.vercel.app/resources/user#nameplate-color-palette
	pub expires_at: Option<u32>,
}

pub enum NameplateColorPalette {
	none,
	crimson,
	berry,
	sky,
	teal,
	forest,
	bubble_gum,
	violet,
	cobalt,
	clover,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileMetadata {
	pub guild_id:                       Snowflake,
	pub pronouns:                       String,
	pub bio:                            String,
	pub banner:                         Option<String>,
	pub accent_color:                   Option<u32>,
	pub theme_colors:                   Option<Vec<(u32, u32)>>,
	pub popout_animation_particle_type: Option<Snowflake>,
	pub emoji:                          Option<Emoji>,
	pub profile_effect:                 Option<ProfileEffect>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileEffect {
	pub id:         Snowflake,
	pub expires_at: Option<u32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PrimaryGuild {
	pub identity_enabled:  Option<bool>,
	pub identity_guild_id: Option<String>,
	pub tag:               Option<String>,
	pub badge:             Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Authenticator {
	pub id:     String,
	pub r#type: String,
	pub name:   String,
}

pub enum AuthenticatorType {
	WEBAUTHN,
	TOTP,
	SMS,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BackupCode {
	pub user_id:  Snowflake,
	pub code:     String,
	pub consumed: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DataHarvest {
	pub harvest_id:       Snowflake,
	pub user_id:          Snowflake,
	pub email:            String,
	/// https://docs.discord.sex/resources/user#harvest-state
	pub state:            String,
	/// https://docs.discord.sex/resources/user#harvest-status
	pub status:           String,
	pub created_at:       Timestamp,
	pub completed_at:     Option<Timestamp>,
	pub polled_at:        Option<Timestamp>,
	/// https://docs.discord.sex/resources/user#harvest-backend-internal-type and https://docs.discord.sex/resources/user#harvest-backend-state
	pub backends:         HashMap<String, String>,
	pub updated_at:       Timestamp,
	pub shadow_run:       bool,
	pub harvest_metadata: HarvestMetadata,
}

pub enum HarvestState {
	INCOMPLETE,
	DELIVERED,
	CANCELLED,
}

pub enum HarvestStatus {
	QUEUED,
	RUNNING,
	FAILED,
	COMPLETED,
	CANCELLED,
}

pub enum HarvestBackendType {
	users,
	analytics,
	activities_e,
	activities_w,
	messages,
	hubspot,
	guilds,
}

pub enum HarvestBackendState {
	INITIAL,
	RUNNING,
	EXTRACTED,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HarvestMetadata {
	pub user_is_staff:   bool,
	pub sla_email_sent:  bool,
	pub bypass_cooldown: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserSurvey {
	pub id:                 Snowflake,
	pub key:                Snowflake,
	pub prompt:             String,
	pub cta:                String,
	pub url:                String,
	/// https://docs.discord.sex/resources/user#survey-requirement-type
	pub guild_requirements: Vec<String>,
	pub guild_size:         Vec<(Option<u32>, Option<u32>)>,
	pub guild_permissions:  Vec<String>,
}

pub enum SurveryRequirementType {
	IS_OWNER,
	IS_ADMIN,
	IS_COMMUNITY,
	GUILD_SIZE,     // requires guild_size
	GUILD_SIZE_ALL, // requires guild_size
	IS_HUB,
	IS_VIEWING,
	GUILD_PERMISSIONS, // requires guild_permissions
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Connection {
	pub id:                  String,
	pub r#type:              String,
	pub name:                String,
	pub verified:            bool,
	pub metadata:            String, // ???? type is "object" and description is "Service-specific metadata about the connection"
	pub metadata_visibility: u8,     // https://docs.discord.sex/resources/user#visibility-type
	pub revoked:             bool,
	pub integrations:        Vec<ConnectionIntegration>,
	pub friend_sync:         bool,
	pub show_activity:       bool,
	pub two_way_link:        bool,
	/// https://docs.discord.sex/resources/user#visibility-type
	pub visibility:          u8,
	pub access_token:        String,
}

pub enum VisibilityType {
	NONE,
	EVERYONE,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConnectionIntegration {
	pub id:      Snowflake,
	pub r#type:  String,
	pub account: IntegrationAccount,
	pub guild:   IntegrationGuild,
}

pub enum ConnectionType {
	amazon_music,
	battlenet,
	bluesky,
	bungie,
	contacts,
	crunchyroll,
	domain,
	ebay,
	epicgames,
	facebook,
	github,
	instagram,
	leagueoflegends,
	mastodon,
	paypal,
	playstation,
	playstation_stg,
	reddit,
	roblox,
	riotgames,
	samsung,
	soundcloud,
	spotify,
	skype,
	steam,
	tiktok,
	twitch,
	twitter,
	xbox,
	youtube,
	unknown,
}
