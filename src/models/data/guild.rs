#![allow(non_camel_case_types)]

use serde::Deserialize;

use super::auto_moderation::AutomodIncedentsData;
use super::clan::Clan;
use super::emoji::Emoji;
use super::integration::IntegrationApplication;
use super::sticker::Sticker;
use super::user::{AvatarDecorationData, User};
use crate::models::types::{Snowflake, Timestamp};

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Guild {
	pub id:                            Snowflake,
	pub name:                          String,
	pub icon:                          Option<String>,
	pub banner:                        Option<String>,
	pub home_header:                   Option<String>,
	pub splash:                        Option<String>,
	pub discovery_splash:              Option<String>,
	pub owner_id:                      Snowflake,
	pub application_id:                Option<Snowflake>,
	pub description:                   Option<String>,
	#[deprecated]
	pub region:                        Option<String>,
	pub afk_channel_id:                Option<Snowflake>,
	pub afk_timeout:                   u16,
	pub widget_enabled:                bool,
	pub widget_channel_id:             Option<Snowflake>,
	/// https://docs.discord.sex/resources/guild#verification-level
	pub verification_level:            u8,
	/// https://docs.discord.sex/resources/guild#message-notification-level
	pub default_message_notifications: u8,
	pub explicit_content_filter:       u8, //https://docs.discord.sex/resources/guild#explicit-content-filter-level
	/// https://docs.discord.sex/resources/guild#guild-features
	pub features:                      Vec<String>,
	pub roles:                         Vec<Role>,
	pub emojis:                        Vec<Emoji>,
	pub stickers:                      Vec<Sticker>,
	/// https://docs.discord.sex/resources/guild#mfa-level
	pub mfa_level:                     u8,
	pub system_channel_id:             Option<Snowflake>,
	pub public_updates_channel_id:     Option<Snowflake>,
	pub safety_alerts_channel_id:      Option<Snowflake>,
	pub max_presences:                 Option<u32>,
	pub max_members:                   u32,
	pub vanity_url_code:               Option<String>,
	pub premium_tier:                  u8, //https://docs.discord.sex/resources/guild#premium-tier
	pub premium_subscription_count:    u32,
	pub preferred_locale:              String,
	pub max_video_channel_users:       u16,
	pub max_stage_video_channel_users: u16,
	#[deprecated]
	pub nsfw:                          bool,
	/// https://docs.discord.sex/resources/guild#nsfw-level
	pub nsfw_level:                    u8,
	/// https://docs.discord.sex/resources/guild#hub-type
	pub hub_type:                      Option<u8>,
	pub premium_progress_bar_enabled:  bool,
	pub latest_onboarding_question_id: Option<Snowflake>,
	pub incidents_data:                Option<AutomodIncedentsData>,
	pub approximate_member_count:      u32,
	pub approximate_presence_count:    u32,
	pub clan:                          Clan,
}

pub enum MessageNotificationLevel {
	ALL_MESSAGES = 0,
	ONLY_MENTIONS = 1,
	NO_MESSAGES = 2,
	INHERIT = 3,
}

pub enum ExplicitContentFilter {
	DISABLED = 0,
	MEMBERS_WITHOUT_ROLES = 1,
	ALL_MEMBERS = 2,
}

pub enum MFALevel {
	NONE = 0,
	ELEVATED = 1,
}

pub enum VerificationLevel {
	NONE = 0,
	LOW = 1,
	MEDIUM = 2,
	HIGH = 3,
	VERY_HIGH = 4,
}

pub enum NsfwLevel {
	DEFAULT = 0,
	EXPLICIT = 1,
	SAFE = 2,
	AGE_RESTRICTED = 3,
}

pub enum PremiumTier {
	NONE = 0,
	TIER_1 = 1,
	TIER_2 = 2,
	TIER_3 = 3,
}

pub enum SystemChannelFlags {
	SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0,
	SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1,
	SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2,
	SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3,
	SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4,
	SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5,
	SUPPRESS_CHANNEL_PROMPT_DEADCHAT = 1 << 7,
}

pub enum PrivacyLevel {
	PUBLIC = 1,
	GUILD_ONLY = 2,
}

pub enum HubType {
	DEFAULT = 0,
	HIGH_SCHOOL = 1,
	COLLEGE = 2,
}

pub enum GuildFeatures {
	ACTIVITIES_ALPHA,
	ACTIVITIES_EMPLOYEE,
	ACTIVITIES_INTERNAL_DEV,
	ACTIVITY_FEED_DISABLED_BY_USER,
	ACTIVITY_FEED_ENABLED_BY_USER,
	ANIMATED_BANNER,
	ANIMATED_ICON,
	// APPLICATION_COMMAND_PERMISSIONS_V2,
	AUTO_MODERATION,
	// AUTOMOD_TRIGGER_KEYWORD_FILTER,
	// AUTOMOD_TRIGGER_ML_SPAM_FILTER,
	// AUTOMOD_TRIGGER_SPAM_LINK_FILTER,
	AUTOMOD_TRIGGER_USER_PROFILE,
	BANNER,
	BFG,
	// BOOSTING_TIERS_EXPERIMENT_MEDIUM_GUILD,
	// BOOSTING_TIERS_EXPERIMENT_SMALL_GUILD,
	BOT_DEVELOPER_EARLY_ACCESS,
	// BURST_REACTIONS,
	// CHANNEL_BANNER,
	CHANNEL_ICON_EMOJIS_GENERATED,
	CHANNEL_HIGHLIGHTS,
	CHANNEL_HIGHLIGHTS_DISABLED,
	CLAN,
	CLAN_DISCOVERY_DISABLED,
	// CLAN_PILOT_GENSHIN,
	// CLAN_PILOT_VALORANT,
	// CLAN_PREPILOT_GENSHIN,
	// CLAN_PREPILOT_VALORANT,
	CLAN_SAFETY_REVIEW_DISABLED,
	// CLYDE_DISABLED,
	// CLYDE_ENABLED,
	// CLYDE_EXPERIMENT_ENABLED,
	COMMERCE,
	COMMUNITY,
	COMMUNITY_CANARY,
	COMMUNITY_EXP_LARGE_GATED,
	COMMUNITY_EXP_LARGE_UNGATED,
	COMMUNITY_EXP_MEDIUM,
	CREATOR_ACCEPTED_NEW_TERMS,
	CREATOR_MONETIZABLE,
	CREATOR_MONETIZABLE_DISABLED,
	CREATOR_MONETIZABLE_PENDING_NEW_OWNER_ONBOARDING,
	CREATOR_MONETIZABLE_PROVISIONAL,
	CREATOR_MONETIZABLE_RESTRICTED,
	CREATOR_MONETIZABLE_WHITEGLOVE,
	CREATOR_MONETIZATION_APPLICATION_ALLOWLIST,
	CREATOR_STORE_PAGE,
	DEVELOPER_SUPPORT_SERVER,
	DISCOVERABLE,
	DISCOVERABLE_DISABLED,
	ENABLED_DISCOVERABLE_BEFORE,
	ENABLED_MODERATION_EXPERIENCE_FOR_NON_COMMUNITY,
	EXPOSED_TO_ACTIVITIES_WTP_EXPERIMENT,
	// EXPOSED_TO_BOOSTING_TIERS_EXPERIMENT,
	// FEATURABLE,
	// FORCE_RELAY,
	FORWARDING_DISABLED,
	// GENSHIN_L30,
	GUESTS_ENABLED,
	// GUILD_AUTOMOD_DEFAULT_LIST,
	// GUILD_COMMUNICATION_DISABLED_GUILDS,
	// GUILD_HOME_DEPRECATION_OVERRIDE,
	// GUILD_HOME_OVERRIDE,
	// GUILD_HOME_TEST,
	// GUILD_MEMBER_VERIFICATION_EXPERIMENT,
	GUILD_ONBOARDING,
	// GUILD_ONBOARDING_ADMIN_ONLY,
	GUILD_ONBOARDING_EVER_ENABLED,
	GUILD_ONBOARDING_HAS_PROMPTS,
	GUILD_PRODUCTS,
	GUILD_PRODUCTS_ALLOW_ARCHIVED_FILE,
	// GUILD_ROLE_SUBSCRIPTIONS,
	// GUILD_ROLE_SUBSCRIPTION_PURCHASE_FEEDBACK_LOOP,
	// GUILD_ROLE_SUBSCRIPTION_TIER_TEMPLATE,
	// GUILD_ROLE_SUBSCRIPTION_TRIALS,
	GUILD_SERVER_GUIDE,
	GUILD_WEB_PAGE_VANITY_URL,
	HAD_EARLY_ACTIVITIES_ACCESS,
	HAS_DIRECTORY_ENTRY,
	HIDE_FROM_EXPERIMENT_UI,
	HUB,
	INCREASED_THREAD_LIMIT,
	INTERNAL_EMPLOYEE_ONLY,
	INVITE_SPLASH,
	LEADERBOARD_ENABLED,
	INVITES_DISABLED,
	LINKED_TO_HUB,
	// LURKABLE,
	// MARKETPLACES_CONNECTION_ROLES,
	// MEDIA_CHANNEL_ALPHA,
	// MEMBER_LIST_DISABLED,
	// MEMBER_PROFILES,
	MEMBER_SAFETY_PAGE_ROLLOUT,
	MEMBER_VERIFICATION_GATE_ENABLED,
	MEMBER_VERIFICATION_MANUAL_APPROVAL,
	MEMBER_VERIFICATION_ROLLOUT_TEST,
	// MOBILE_WEB_ROLE_SUBSCRIPTION_PURCHASE_PAGE,
	// MONETIZATION_ENABLED,
	MORE_EMOJI,
	MORE_SOUNDBOARD,
	MORE_STICKERS,
	NEWS,
	// NEW_THREAD_PERMISSIONS,
	NON_COMMUNITY_RAID_ALERTS,
	PARTNERED,
	PREMIUM_TIER_3_OVERRIDE,
	PREVIEW_ENABLED,
	// PRIVATE_THREADS,
	PRODUCTS_AVAILABLE_FOR_PURCHASE,
	// PUBLIC,
	// PUBLIC_DISABLED,
	RAID_ALERTS_DISABLED,
	// RAID_ALERTS_ENABLED,
	// RAPIDASH_TEST,
	// RAPIDASH_TEST_REBIRTH,
	RELAY_ENABLED,
	// RESTRICT_SPAM_RISK_GUILDS,
	ROLE_ICONS,
	ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE,
	ROLE_SUBSCRIPTIONS_ENABLED,
	// ROLE_SUBSCRIPTIONS_ENABLED_FOR_PURCHASE,
	// SEVEN_DAY_THREAD_ARCHIVE,
	SHARD,
	SHARED_CANVAS_FRIENDS_AND_FAMILY_TEST,
	SOUNDBOARD,
	// SUMMARIES_ENABLED,
	SUMMARIES_ENABLED_GA,
	SUMMARIES_DISABLED_BY_USER,
	SUMMARIES_ENABLED_BY_USER,
	SUMMARIES_LONG_LOOKBACK,
	SUMMARIES_OPT_OUT_EXPERIENCE,
	STAFF_LEVEL_COLLABORATOR_REQUIRED,
	STAFF_LEVEL_RESTRICTED_COLLABORATOR_REQUIRED,
	// TEXT_IN_STAGE_ENABLED,
	// TEXT_IN_VOICE_ENABLED,
	// THREADS_ENABLED,
	// THREADS_ENABLED_TESTING,
	THREAD_DEFAULT_AUTO_ARCHIVE_DURATION,
	// THREADS_ONLY_CHANNEL,
	// THREE_DAY_THREAD_ARCHIVE,
	// TICKETED_EVENTS_ENABLED,
	// TICKETING_ENABLED,
	// VALORANT_L30,
	VANITY_URL,
	VERIFIED,
	VIP_REGIONS,
	// VOICE_CHANNEL_EFFECTS,
	VOICE_IN_THREADS,
	WELCOME_SCREEN_ENABLED,
}

// can be changed via https://docs.discord.sex/resources/guild#modify-guild
pub enum ModifiableGuildFeatures {
	ACTIVITY_FEED_DISABLED_BY_USER,
	ACTIVITY_FEED_ENABLED_BY_USER,
	// CLYDE_DISABLED,
	// CLYDE_ENABLED,
	COMMUNITY,
	DISCOVERABLE,
	ENABLED_MODERATION_EXPERIENCE_FOR_NON_COMMUNITY,
	INVITES_DISABLED,
	MEMBER_VERIFICATION_GATE_ENABLED,
	NON_COMMUNITY_RAID_ALERTS,
	RAID_ALERTS_DISABLED,
	SUMMARIES_ENABLED_BY_USER,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct UserGuild {
	pub id:                         Snowflake,
	pub name:                       String,
	pub icon:                       Option<String>,
	pub banner:                     Option<String>,
	pub owner:                      bool,
	pub features:                   Vec<String>, // enabled guild features, should replace with enum
	pub permissions:                String,
	pub approximate_member_count:   u32,
	pub approximate_presence_count: u32,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GuildWidget {
	pub id:             Snowflake,
	pub name:           String,
	pub instant_invite: Option<String>,
	pub presence_count: u32,
	pub channels:       Vec<WidgetChannel>,
	pub members:        Vec<WidgetMember>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct WidgetChannel {
	pub id:       Snowflake,
	pub name:     String,
	pub position: u16,
}

// Due to privacy concerns, id, discriminator, and avatar are anonymized. id is replaced with an incrementing integer, discriminator is always 0000, and avatar is always null (replaced with an encrypted avatar_url field).
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct WidgetMember {
	pub id:         Snowflake,
	pub username:   String,
	pub avatar_url: String,
	pub status:     String,
	pub activity:   WidgetMemberActivity,
	pub channel_id: Snowflake,
	pub deaf:       bool,
	pub mute:       bool,
	pub self_deaf:  bool,
	pub self_mute:  bool,
	pub suppress:   bool,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct WidgetMemberActivity {
	pub name: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GuildWidgetSettings {
	pub enabled:    bool,
	pub channel_id: Option<Snowflake>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Role {
	pub id:            Snowflake,
	pub name:          String,
	pub description:   Option<String>,
	pub color:         u32,
	pub hoist:         bool,
	pub icon:          Option<String>,
	pub unicode_emoji: Option<String>,
	pub position:      u16,
	pub permissions:   String,
	pub managed:       bool,
	pub mentionable:   bool,
	/// https://docs.discord.sex/resources/guild#role-flags
	pub flags:         u8,
	pub tags:          RoleTags,
}

pub enum RoleFlags {
	IN_PROMPT,
}

// all Option<String> in this object will be null if true and empty if false for whatever reason
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RoleTags {
	pub bot_id:                  Snowflake,
	pub integration_id:          Snowflake,
	pub premium_subscriber:      Option<String>,
	pub subscription_listing_id: Snowflake,
	pub available_for_purchase:  Option<String>,
	pub guild_connections:       Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RoleConnectionConfig {
	pub connection_type:           String,
	pub connection_metadata_field: Option<String>,
	/// https://docs.discord.sex/resources/guild#role-connection-operator-type
	pub operator:                  Option<u8>,
	pub value:                     Option<String>,
	pub application_id:            Snowflake,
	pub application:               IntegrationApplication,
	pub name:                      String,
	pub description:               String,
	pub result:                    bool,
}

pub enum RoleConnectionOperatorType {
	INTEGER_LESS_THAN_OR_EQUAL,
	INTEGER_GREATER_THAN_OR_EQUAL,
	INTEGER_EQUAL,
	INTEGER_NOT_EQUAL,
	DATETIME_LESS_THAN_OR_EQUAL,
	DATETIME_GREATER_THAN_OR_EQUAL,
	BOOLEAN_EQUAL,
	BOOLEAN_NOT_EQUAL,
}

#[derive(Deserialize, Default)]
#[serde(default)]
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
	/// https://docs.discord.sex/resources/guild#guild-member-flags
	pub flags:                        u16,
	pub permissions:                  String,
}

pub enum GuildMemberFlags {
	DID_REJOIN,
	COMPLETED_ONBOARDING,
	BYPASSES_VERIFICATION,
	STARTED_ONBOARDING,
	IS_GUEST,
	STARTED_SERVER_GUIDE,
	COMPLETED_SERVER_GUIDE,
	AUTOMOD_QUARANTINED_NAME,
	// AUTOMOD_QUARANTINED_BIO,
	DM_SETTINGS_UPSELL_ACKNOWLEDGED,
	AUTOMOD_QUARANTINED_CLAN_TAG,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct SupplementalGuildMember {
	pub user_id:            Snowflake,
	pub member:             GuildMember,
	pub join_source_type:   u8,
	pub source_invite_code: Option<String>,
	pub inviter_id:         Option<Snowflake>,
	pub integration_type:   Option<u8>,
}

pub enum JoinSource {
	UNSPECIFIED,
	BOT,
	INTEGRATION,
	DISCOVERY,
	HUB,
	INVITE,
	VANITY_URL,
	MANUAL_MEMBER_VERIFICATION,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Ban {
	pub user:   User,
	pub reason: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct WelcomeScreen {
	pub description:      Option<String>,
	pub welcome_channels: Vec<WelcomeChannel>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct WelcomeChannel {
	pub channel_id:  Snowflake,
	pub description: String,
	pub emoji_id:    Option<Snowflake>,
	pub emoji_name:  Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct MemberVerification {
	pub version:     Option<Timestamp>,
	pub form_fields: Vec<MemberVerificationForm>,
	pub description: Option<String>,
	pub guild:       Option<MemberVerificationGuild>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct MemberVerificationForm {
	/// https://docs.discord.sex/resources/guild#member-verification-form-field-type
	pub field_type:  String,
	pub label:       String,
	pub choices:     Vec<String>,
	pub values:      Option<Vec<String>>,
	pub response:    Option<ResponseType>,
	pub required:    bool,
	pub description: Option<String>,
	pub automations: Option<Vec<String>>,
	pub placeholder: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ResponseType {
	String(String),
	Bool(bool),
	Integer(u32),
}

pub enum MemberVerificationFormFieldType {
	TERMS,
	TEXT_INPUT,
	PARAGRAPH,
	MULTIPLE_CHOICE,
	// VERIFICATION,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct MemberVerificationGuild {
	pub id:                         Snowflake,
	pub name:                       String,
	pub icon:                       Option<String>,
	pub description:                Option<String>,
	pub splash:                     Option<String>,
	pub discovery_splash:           Option<String>,
	pub home_header:                Option<String>,
	pub verification_level:         u8,
	pub features:                   Vec<String>,
	pub emojis:                     Vec<Emoji>,
	pub approximate_member_count:   u32,
	pub approximate_presence_count: u32,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GuildJoinRequest {
	pub id:                   Snowflake,
	pub join_request_id:      Snowflake,
	pub created_at:           Timestamp,
	pub application_status:   String,
	pub guild_id:             Snowflake,
	pub form_responses:       MemberVerificationForm,
	pub last_seen:            Option<Timestamp>,
	pub actioned_at:          Snowflake,
	pub actioned_by_user:     User,
	pub rejection_reason:     Option<String>,
	pub user_id:              Snowflake,
	pub user:                 User,
	pub interview_channel_id: Option<Snowflake>,
}

pub enum GuildJoinRequestStatus {
	STARTED,
	SUBMITTED,
	REJECTED,
	APPROVED,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Onboarding {
	pub guild_id:            Snowflake,
	pub prompts:             Vec<OnboardingPrompt>,
	pub default_channel_ids: Vec<Snowflake>,
	pub enabled:             bool,
	pub below_requirements:  bool,
	pub mode:                u8,
}

pub enum OnboardingMode {
	ONBOARDING_DEFAULT,
	ONBOARDING_ADVANCED,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct OnboardingPrompt {
	pub id:            Snowflake,
	pub r#type:        u8,
	pub options:       Vec<OnboardingPromptOption>,
	pub title:         String,
	pub single_select: bool,
	pub required:      bool,
	pub in_onboarding: bool,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct OnboardingPromptOption {
	pub id:          Snowflake,
	pub channel_ids: Vec<Snowflake>,
	pub role_ids:    Vec<Snowflake>,
	pub emoji:       Emoji,
	pub title:       String,
	pub description: Option<String>,
}

pub enum OnboardingPromptType {
	MULTIPLE_CHOICE,
	DROPDOWN,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct PremiumGuildSubscription {
	pub id:            Snowflake,
	pub guild_id:      Snowflake,
	pub uder_id:       Snowflake,
	pub ended:         bool,
	pub ends_at:       Timestamp,
	pub pause_ends_at: Option<Timestamp>,
	pub user:          User,
}
