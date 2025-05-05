#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::integration::Integration;
use super::team::{Company, Team};
use super::user::User;
use crate::bitflags;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Application {
	pub id:                                Snowflake,
	pub name:                              String,
	pub description:                       String,
	pub icon:                              Option<String>,
	pub cover_image:                       Option<String>,
	pub splash:                            String,
	/// https://docs.discord.sex/resources/application#application-type
	pub r#type:                            Option<u8>,
	/// https://docs.discord.sex/resources/application#application-flags
	pub flags:                             u64,
	pub primary_sku_id:                    Snowflake,
	pub verify_key:                        String,
	pub guild_id:                          Snowflake,
	pub eula_id:                           Snowflake,
	pub slug:                              String,
	pub aliases:                           Vec<String>,
	pub executables:                       Vec<ApplicationExecutable>,
	pub third_party_skus:                  Vec<ApplicationSKU>,
	pub hook:                              bool,
	pub overlay:                           bool,
	/// https://docs.discord.sex/resources/application#overlay-method-flags
	pub overlay_methods:                   u8,
	pub overlay_warn:                      bool,
	pub overlay_compatibility_hook:        bool,
	pub bot:                               User,
	pub owner:                             User,
	pub team:                              Team,
	pub developers:                        Vec<Company>,
	pub publishers:                        Vec<Company>,
	pub rpc_origins:                       Vec<String>,
	pub redirect_uris:                     Vec<String>,
	pub deeplink_uri:                      String,
	pub integration_public:                bool,
	pub integration_require_code_grant:    bool,
	#[deprecated]
	pub bot_public:                        bool,
	#[deprecated]
	pub bot_require_code_grant:            bool,
	pub bot_disabled:                      bool,
	pub bot_quarantined:                   bool,
	pub approximate_guild_count:           u32,
	pub approximate_user_install_count:    u32,
	/// https://docs.discord.sex/resources/application#internal-guild-restriction
	pub internal_guild_restriction:        u8,
	pub terms_of_service_url:              String,
	pub privacy_policy_url:                String,
	pub role_connections_verification_url: Option<String>,
	pub interactions_endpoint_url:         String,
	/// https://docs.discord.sex/resources/application#application-interactions-version
	pub interactions_version:              u8,
	pub interactions_event_types:          Vec<String>,
	/// https://docs.discord.sex/resources/application#event-webhooks-status
	pub event_webhooks_status:             u8,
	pub event_webhooks_url:                String,
	/// https://docs.discord.sex/resources/application#event-webhooks-type
	pub event_webhooks_types:              Vec<String>,
	/// https://docs.discord.sex/resources/application#explicit-content-filter-level
	pub explicit_content_filter:           u8,
	pub tags:                              Vec<String>,
	pub install_params:                    ApplicationInstallParams,
	pub custom_install_url:                String,
	pub integration_types_config:          HashMap<u32, Option<ApplicationIntegrationType>>,
	pub is_verified:                       bool,
	/// https://docs.discord.sex/resources/application#application-verification-state
	pub verification_state:                u8,
	/// https://docs.discord.sex/resources/application#store-application-state
	pub store_application_state:           u8,
	/// https://docs.discord.sex/resources/application#rpc-application-state
	pub rpc_application_state:             u8,
	/// https://docs.discord.sex/resources/application#creator-monetization-state
	pub creator_monetization_state:        u32,
	pub is_discoverable:                   bool,
	/// https://docs.discord.sex/resources/application#application-discoverability-state
	pub discoverability_state:             u8,
	/// https://docs.discord.sex/resources/application#application-discovery-eligibility-flags
	pub discovery_eligibility_flags:       u64,
	pub is_monetized:                      bool,
	pub storefront_available:              bool,
	/// https://docs.discord.sex/resources/application#application-monetization-state
	pub monetization_state:                u8,
	pub monetization_eligibility_flags:    u64,
	pub max_participants:                  i32,
	pub embedded_activity_config:          EmbeddedActivityConfig,
}

pub enum ApplicationType {
	GAME = 1,
	// MUSIC = 2,
	TICKETED_EVENTS = 3,
	CREATOR_MONETIZATION = 4,
}

bitflags! {
  pub struct ApplicationFlags: u64 {
	const EMBEDDED_RELEASED = 1 << 1;
	const MANAGED_EMOJI = 1 << 2;
	const EMBEDDED_IAP = 1 << 3;
	const GROUP_DM_CREATE = 1 << 4;
	// const RPC_PRIVATE_BETA = 1 << 5;
	const AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
	const GAME_PROFILE_DISABLED = 1 << 7;
	const PUBLIC_OAUTH2_CLIENT = 1 << 8;
	const CONTEXTLESS_ACTIVITY = 1 << 9;
	const SOCIAL_LAYER_INTEGRATION_LIMITED = 1 << 10;
	// const ALLOW_ASSETS = 1 << 8;
	// const ALLOW_ACTIVITY_ACTION_SPECTATE = 1 << 9;
	// const ALLOW_ACTIVITY_ACTION_JOIN_REQUEST = 1 << 10;
	// const RPC_HAS_CONNECTED = 1 << 11;
	const GATEWAY_PRESENCE = 1 << 12;
	const GATEWAY_PRESENCE_LIMITED = 1 << 13;
	const GATEWAY_GUILD_MEMBERS = 1 << 14;
	const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
	const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
	const EMBEDDED = 1 << 17;
	const GATEWAY_MESSAGE_CONTENT = 1 << 18;
	const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
	const EMBEDDED_FIRST_PARTY = 1 << 20;
	const APPLICATION_COMMAND_MIGRATED = 1 << 21;
	const APPLICATION_COMMAND_BADGE = 1 << 23;
	const ACTIVE = 1 << 24;
	const ACTIVE_GRACE_PERIOD = 1 << 25;
	const IFRAME_MODAL = 1 << 26;
	const SOCIAL_LAYER_INTEGRATION = 1 << 27;
	const PROMOTED = 1 << 29;
	const PARTNER = 1 << 30;
  }
}

bitflags! {
  pub struct ApplicationOverlayMethods: u64 {
	const OUT_OF_PROCESS = 1 << 0;
  }
}

pub enum ApplicationInternalGuildRestriction {
	JOIN_ALL = 1,
	JOIN_EXTERNAL_ONLY = 2,
	JOIN_INTERNAL_ONLY = 3,
}

pub enum ApplicationInteractionsVersion {
	VERSION_1 = 1,
	VERSION_2 = 2,
}

pub enum ApplicationEventWebhookStatus {
	DISABLED = 1,
	ENABLED = 2,
}

pub enum ApplicationEventWebhook {
	APPLICATION_AUTHORIZED,
	ENTITLEMENT_CREATE,
	QUEST_USER_ENROLLMENT,
}

pub enum ApplicationExplicitContentFilterLevel {
	INHERIT = 0,
	ALWAYS = 1,
}

pub enum ApplicationVerificationState {
	INELIGIBLE = 1,
	UNSUBMITTED = 2,
	SUBMITTED = 3,
	SUCCEEDED = 4,
}

pub enum ApplicationStoreState {
	NONE = 1,
	PAID = 2,
	SUBMITTED = 3,
	APPROVED = 4,
	REJECTED = 5,
}

pub enum ApplicationRpcState {
	DISABLED = 0,
	UNSUBMITTED = 1,
	SUBMITTED = 2,
	APPROVED = 3,
	REJECTED = 4,
}

bitflags! {
  pub struct ApplicationCreatorMonetizationState: u64 {
	const VERIFIED = 1 << 0;
	const HAS_TEAM = 1 << 1;
	const APPROVED_COMMANDS = 1 << 2;
	const TERMS_OF_SERVICE = 1 << 3;
	const PRIVACY_POLICY = 1 << 4;
	const SAFE_NAME = 1 << 5;
	const SAFE_DESCRIPTION = 1 << 6;
	const SAFE_ROLE_CONNECTIONS = 1 << 7;
	const USER_IS_TEAM_OWNER = 1 << 8;
	const NOT_QUARANTINED = 1 << 9;
	const USER_LOCALE_SUPPORTED = 1 << 10;
	const USER_AGE_SUPPORTED = 1 << 11;
	const USER_DATE_OF_BIRTH_DEFINED = 1 << 12;
	const USER_MFA_ENABLED = 1 << 13;
	const USER_EMAIL_VERIFIED = 1 << 14;
	const TEAM_MEMBERS_EMAIL_VERIFIED = 1 << 15;
	const TEAM_MEMBERS_MFA_ENABLED = 1 << 16;
	const NO_BLOCKING_ISSUES = 1 << 17;
		const VALID_PAYOUT_STATUS = 1 << 18;
  }
}

pub enum ApplicationDiscoverabilityState {
	INELIGIBLE = 1,
	NOT_DISCOVERABLE = 2,
	DISCOVERABLE = 3,
	FEATUREABLE = 4,
	BLOCKED = 5,
}

bitflags! {
  pub struct ApplicationDiscoverabilityFlags: u64 {
	const VERIFIED = 1 << 0;
	const TAG = 1 << 1;
	const DESCRIPTION = 1 << 2;
	const TERMS_OF_SERVICE = 1 << 3;
	const PRIVACY_POLICY = 1 << 4;
	const INSTALL_PARAMS = 1 << 5;
	const SAFE_NAME = 1 << 6;
	const SAFE_DESCRIPTION = 1 << 7;
	const APPROVED_COMMANDS = 1 << 8;
	const SUPPORT_GUILD = 1 << 9;
	const SAFE_COMMANDS = 1 << 10;
	const MFA = 1 << 11;
	const SAFE_DIRECTORY_OVERVIEW = 1 << 12;
	const SUPPORTED_LOCALES = 1 << 13;
	const SAFE_SHORT_DESCRIPTION = 1 << 14;
		const SAFE_ROLE_CONNECTIONS = 1 << 15;
  }
}

pub enum ApplicationMonetizationState {
	NONE = 1,
	ENABLED = 2,
	BLOCKED = 3,
}

bitflags! {
  pub struct ApplicationMonetizationFlags: u64 {
	const VERIFIED = 1 << 0;
	const HAS_TEAM = 1 << 1;
	const APPROVED_COMMANDS = 1 << 2;
	const TERMS_OF_SERVICE = 1 << 3;
	const PRIVACY_POLICY = 1 << 4;
	const SAFE_NAME = 1 << 5;
	const SAFE_DESCRIPTION = 1 << 6;
	const SAFE_ROLE_CONNECTIONS = 1 << 7;
	const USER_IS_TEAM_OWNER = 1 << 8;
	const NOT_QUARANTINED = 1 << 9;
	const USER_LOCALE_SUPPORTED = 1 << 10;
	const USER_AGE_SUPPORTED = 1 << 11;
	const USER_DATE_OF_BIRTH_DEFINED = 1 << 12;
	const USER_MFA_ENABLED = 1 << 13;
	const USER_EMAIL_VERIFIED = 1 << 14;
	const TEAM_MEMBERS_EMAIL_VERIFIED = 1 << 15;
	const TEAM_MEMBERS_MFA_ENABLED = 1 << 16;
	const NO_BLOCKING_ISSUES = 1 << 17;
		const VALID_PAYOUT_STATUS = 1 << 18;
  }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationExecutable {
	pub os:          String,
	pub name:        String,
	pub is_launcher: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationSKU {
	pub id:          Option<String>,
	pub sku:         Option<String>,
	/// https://docs.discord.sex/resources/application#distributor-type
	pub distributor: String,
}

pub enum DistributorType {
	discord,
	steam,
	twitch,
	uplay,
	battlenet,
	origin,
	gog,
	epic,
	google_play,
}

#[derive(Deserialize, Default, Serialize)]
#[serde(default)]
pub struct ApplicationInstallParams {
	/// https://docs.discord.sex/topics/oauth2#oauth2-scopes
	pub scopes:      Vec<String>,
	/// https://docs.discord.sex/topics/permissions
	pub permissions: String,
}

#[derive(Serialize, Deserialize, Default)]
pub enum ApplicationIntegrationType {
	GUILD_INSTALL = 0,
	USER_INSTALL = 1,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationIntegrationTypeConfig {
	pub oauth2_install_params: ApplicationInstallParams,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationProxyConfig {
	pub url_map: ApplicationProxyMap,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationProxyMap {
	pub prefix: String,
	pub target: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmbeddedActivityConfig {
	pub application_id:                        Snowflake,
	pub activity_preview_video_asset_id:       Option<Snowflake>,
	/// https://docs.discord.sex/resources/application#embedded-activity-platform-type
	pub supported_platforms:                   Vec<String>,
	/// https://docs.discord.sex/resources/application#embedded-activity-orientation-lock-state-type
	pub default_orientation_lock_state:        u8,
	/// https://docs.discord.sex/resources/application#embedded-activity-orientation-lock-state-type
	pub tablet_default_orientation_lock_state: u8,
	pub requires_age_gate:                     bool,
	pub legacy_responsive_aspect_ratio:        bool,
	#[deprecated]
	/// https://docs.discord.sex/resources/guild#premium-tier
	pub premium_tier_requirement:              Option<u8>,
	#[deprecated]
	pub free_period_starts_at:                 Option<Timestamp>,
	#[deprecated]
	pub free_period_ends_at:                   Option<Timestamp>,
	pub client_platform_config:                HashMap<String, EmbeddedActivityPlatformConfig>,
	pub shelf_rank:                            u16,
	pub has_csp_exception:                     bool,
	pub displays_advertisements:               bool,
}

pub enum EmbeddedActivitySupportedPlatformType {
	web,
	android,
	ios,
}

pub enum EmbeddedActivityOrientationLockStateType {
	UNLOCKED = 1,
	PORTRAIT = 2,
	LANDSCAPE = 3,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmbeddedActivityPlatformConfig {
	/// https://docs.discord.sex/resources/application#embedded-activity-label-type
	pub label_type:    u8,
	pub label_until:   Option<Timestamp>,
	/// https://docs.discord.sex/resources/application#embedded-activity-release-phase
	pub release_phase: String,
}

pub enum EmbeddedActivityLabelType {
	NONE = 0,
	NEW = 1,
	UPDATED = 2,
}

pub enum EmbeddedActivityReleasePhase {
	in_development,
	activities_team,
	employee_release,
	soft_launch,
	global_launch,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationAsset {
	pub id:     String,
	pub r#type: u8, // kinda useless its either just 1 or 2 and no one knows what its for https://docs.discord.sex/resources/application#application-asset-type
	pub name:   String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ExternalAsset {
	pub url:                 String,
	pub external_asset_path: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationRoleConnection {
	pub platform_name:        Option<String>,
	pub platform_username:    Option<String>,
	pub metadata:             ApplicationRoleConnectionMetadata,
	pub application:          Integration,
	pub application_metadata: Vec<ApplicationRoleConnectionMetadata>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationRoleConnectionMetadata {
	/// https://docs.discord.sex/resources/guild#role-connection-operator-type
	pub r#type:                    u8,
	pub key:                       String,
	pub name:                      String,
	pub name_localizations:        HashMap<String, String>,
	pub description:               String,
	pub description_localizations: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DetectableApplication {
	pub id:                         Snowflake,
	pub name:                       String,
	pub aliases:                    Vec<String>,
	pub executables:                Vec<ApplicationExecutable>,
	pub themes:                     Vec<String>,
	pub hook:                       bool,
	pub overlay:                    bool,
	/// https://docs.discord.sex/resources/application#overlay-method-flags
	pub overlay_methods:            Option<u8>,
	pub overlay_warn:               bool,
	pub overlay_compatibility_hook: bool,
}

pub enum ApplicationDisclosureType {
	UNSPECIFIED_DISCLOSURE = 0,
	IP_LOCATION = 1,
	DISPLAYS_ADVERTISEMENTS = 2,
	PARTNER_SDK_DATA_SHARING_MESSAGE = 3,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApplicationDistributor {
	pub distributor: String,
	pub sku:         String,
}

pub enum OperatingSystem {
	win32,
	darwin,
	linux,
}

pub enum ApplicationReportMissingDataType {
	icon,
}
