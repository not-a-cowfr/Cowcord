#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::emoji::Emoji;
use super::family_center::LinkedUser;
use super::integration::{IntegrationAccount, IntegrationGuild};
use crate::models::locales::Locales;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct User {
	pub id: Snowflake,
	pub username: String,
	#[deprecated]
	pub discriminator: String,
	pub global_name: Option<String>,
	pub avatar: Option<String>,
	pub avatar_decoration_data: Option<AvatarDecorationData>,
	pub primary_guild: Option<PrimaryGuild>,
	pub linked_users: Vec<LinkedUser>,
	pub bot: bool,
	pub system: bool,
	pub mfa_enabled: bool,
	pub nsfw_allowed: Option<bool>,
	pub age_verification_status: AgeVerificationStatus,
	pub pronouns: String,
	pub bio: String,
	pub banner: Option<String>,
	pub accent_color: Option<u32>,
	pub locale: Locales,
	pub verified: bool,
	pub email: Option<String>,
	pub phone: Option<String>,
	pub premium_type: PremiumType,
	pub personal_connection_id: Snowflake,
	pub flags: UserFlags,
	pub public_flags: u64,
	pub purchased_flags: PurchasedFlags,
	pub premium_flags: PremiumFlags,
	pub desktop: bool,
	pub mobile: bool,
	pub has_bounced_email: bool,
	pub authenticator_types: Vec<AuthenticatorType>,
}

bitflags! {
	pub struct UserFlags: u64 {
		const STAFF = 1 << 0;
		const PARTNER = 1 << 1;
		const HYPESQUAD = 1 << 2;
		const BUG_HUNTER_LEVEL_1 = 1 << 3;
		const MFA_SMS = 1 << 4;
		const PREMIUM_PROMO_DISMISSED = 1 << 5;
		const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
		const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
		const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
		const PREMIUM_EARLY_SUPPORTER = 1 << 9;
		const TEAM_PSEUDO_USER = 1 << 10;
		const IS_HUBSPOT_CONTACT = 1 << 11;
		const SYSTEM = 1 << 12;
		const HAS_UNREAD_URGENT_MESSAGES = 1 << 13;
		const BUG_HUNTER_LEVEL_2 = 1 << 14;
		const UNDERAGE_DELETED = 1 << 15;
		const VERIFIED_BOT = 1 << 16;
		const VERIFIED_DEVELOPER = 1 << 17;
		const CERTIFIED_MODERATOR = 1 << 18;
		const BOT_HTTP_INTERACTIONS = 1 << 19;
		const SPAMMER = 1 << 20;
		const DISABLE_PREMIUM = 1 << 21;
		const ACTIVE_DEVELOPER = 1 << 22;
		const PROVISIONAL_ACCOUNT = 1 << 23;
		const HIGH_GLOBAL_RATE_LIMIT = 1 << 33;
		const DELETED = 1 << 34;
		const DISABLED_SUSPICIOUS_ACTIVITY = 1 << 35;
		const SELF_DELETED = 1 << 36;
		const PREMIUM_DISCRIMINATOR = 1 << 37;
		const USED_DESKTOP_CLIENT = 1 << 38;
		const USED_WEB_CLIENT = 1 << 39;
		const USED_MOBILE_CLIENT = 1 << 40;
		const DISABLED = 1 << 41;
		const HAS_SESSION_STARTED = 1 << 43;
		const QUARANTINED = 1 << 44;
		const PREMIUM_ELIGIBLE_FOR_UNIQUE_USERNAME = 1 << 47;
		const COLLABORATOR = 1 << 50;
		const RESTRICTED_COLLABORATOR = 1 << 51;
	}
}

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

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PremiumType {
	#[deprecated]
	NONE = 0,
	TIER_1 = 1,
	TIER_2 = 2,
	TIER_3 = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AgeVerificationStatus {
	UNVERIFIED = 1,
	VERIFIED_TEEN = 2,
	VERIFIED_ADULT = 3,
}

pub enum RequiredActionType {
	UPDATE_AGREEMENTS,
	COMPLETE_CAPTCHA,
	VERIFY_EMAIL,
	REVERIFY_EMAIL,
	VERIFY_PHONE,
	REVERIFY_PHONE,
	VERIFY_PHONE_THEN_EMAIL,
	VERIFY_EMAIL_OR_VERIFY_PHONE,
	REVERIFY_EMAIL_OR_VERIFY_PHONE,
	VERIFY_EMAIL_OR_REVERIFY_PHONE,
	REVERIFY_EMAIL_OR_REVERIFY_PHONE,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AvatarDecorationData {
	pub asset: String,
	pub sku_id: String,
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
	pub asset: String,
	pub sku_id: Snowflake,
	pub label: String,
	pub palette: NameplateColorPalette,
	pub expires_at: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
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
	pub guild_id: Snowflake,
	pub pronouns: String,
	pub bio: String,
	pub banner: Option<String>,
	pub accent_color: Option<u32>,
	pub theme_colors: Option<Vec<(u32, u32)>>,
	pub popout_animation_particle_type: Option<Snowflake>,
	pub emoji: Option<Emoji>,
	pub profile_effect: Option<ProfileEffect>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileEffect {
	pub id: Snowflake,
	pub expires_at: Option<u32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PrimaryGuild {
	pub identity_enabled: Option<bool>,
	pub identity_guild_id: Option<String>,
	pub tag: Option<String>,
	pub badge: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Authenticator {
	pub id: String,
	pub r#type: String,
	pub name: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AuthenticatorType {
	WEBAUTHN = 1,
	TOTP = 2,
	SMS = 3,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BackupCode {
	pub user_id: Snowflake,
	pub code: String,
	pub consumed: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DataHarvest {
	pub harvest_id: Snowflake,
	pub user_id: Snowflake,
	pub email: String,
	pub state: HarvestState,
	pub status: HarvestStatus,
	pub created_at: Timestamp,
	pub completed_at: Option<Timestamp>,
	pub polled_at: Option<Timestamp>,
	pub backends: HashMap<HarvestBackendType, HarvestBackendState>,
	pub updated_at: Timestamp,
	pub shadow_run: bool,
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
	pub user_is_staff: bool,
	pub sla_email_sent: bool,
	pub bypass_cooldown: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserSurvey {
	pub id: Snowflake,
	pub key: Snowflake,
	pub prompt: String,
	pub cta: String,
	pub url: String,
	pub guild_requirements: Vec<SurveryRequirementType>,
	pub guild_size: Vec<(Option<u32>, Option<u32>)>,
	pub guild_permissions: Vec<String>,
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
	pub id: String,
	pub r#type: String,
	pub name: String,
	pub verified: bool,
	pub metadata: Value,
	pub metadata_visibility: VisibilityType,
	pub revoked: bool,
	pub integrations: Vec<ConnectionIntegration>,
	pub friend_sync: bool,
	pub show_activity: bool,
	pub two_way_link: bool,
	pub visibility: VisibilityType,
	pub access_token: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VisibilityType {
	NONE = 0,
	EVERYONE = 1,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConnectionIntegration {
	pub id: Snowflake,
	pub r#type: String,
	pub account: IntegrationAccount,
	pub guild: IntegrationGuild,
}

#[derive(Serialize, Deserialize)]
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
	#[serde(rename = "playstation-stg")]
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
