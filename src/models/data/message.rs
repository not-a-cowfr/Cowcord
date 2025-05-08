#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::application::Application;
use super::channel::Channel;
use super::emoji::Emoji;
use super::integration::IntegrationApplication;
use super::soundboard::SoundboardSound;
use super::sticker::{Sticker, StickerItem};
use super::user::User;
use crate::models::types::{Snowflake, Timestamp};
use crate::{bitflags, enum_number};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Message {
	pub id:                     Snowflake,
	pub channel_id:             Snowflake,
	pub author:                 User,
	pub content:                String,
	pub timestamp:              Timestamp,
	pub edited_timestamp:       Option<Timestamp>,
	pub tts:                    bool,
	pub mention_everyone:       bool,
	pub mentions:               Vec<User>,
	pub mention_channels:       Vec<Channel>,
	pub attachments:            Vec<MessageAttachment>,
	pub embeds:                 Vec<MessageEmbed>,
	pub reactions:              Vec<MessageReaction>,
	pub nonce:                  NonceResponseType,
	pub pinned:                 bool,
	pub webhook_id:             Snowflake,
	/// https://docs.discord.food/resources/message#message-type
	pub r#type:                 u8,
	pub activity:               MessageActivity,
	pub application:            IntegrationApplication,
	pub application_id:         Snowflake,
	/// https://docs.discord.food/resources/message#message-flags
	pub flags:                  u64,
	pub message_reference:      MessageReference,
	pub referenced_message:     Option<Box<Message>>,
	pub message_snapshots:      Vec<MessageSnapshot>,
	pub call:                   MessageCall,
	#[deprecated]
	pub interaction:            MessageInteraction,
	pub interaction_metadata:   MessageInteractionMetadata,
	pub thread:                 Channel,
	pub role_subscription_data: MessageRoleSubscription,
	pub purchase_notification:  MessagePurchaseNotification,
	pub gift_info:              MessageGiftInfo,
	pub components:             Vec<MessageComponent>,
	pub sticker_items:          Vec<StickerItem>,
	pub stickers:               Vec<Sticker>,
	pub poll:                   Poll,
	pub changelog_id:           Snowflake,
	pub soundboard_sounds:      Vec<SoundboardSound>,
	pub potions:                Vec<Potion>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(untagged)]
enum NonceResponseType {
	String(String),
	Integer(u32),
	#[default]
	None,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum MessageType {
		DEFAULT = 0,
		RECIPIENT_ADD = 1,
		RECIPIENT_REMOVE = 2,
		CALL = 3,
		CHANNEL_NAME_CHANGE = 4,
		CHANNEL_ICON_CHANGE = 5,
		CHANNEL_PINNED_MESSAGE = 6,
		USER_JOIN = 7,
		PREMIUM_GUILD_SUBSCRIPTION = 8,
		PREMIUM_GUILD_SUBSCRIPTION_TIER_1 = 9,
		PREMIUM_GUILD_SUBSCRIPTION_TIER_2 = 10,
		PREMIUM_GUILD_SUBSCRIPTION_TIER_3 = 11,
		CHANNEL_FOLLOW_ADD = 12,
		// GUILD_STREAM = 13,
		GUILD_DISCOVERY_DISQUALIFIED = 14,
		GUILD_DISCOVERY_REQUALIFIED = 15,
		GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING = 16,
		GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING = 171,
		THREAD_CREATED = 18,
		REPLY = 19,
		CHAT_INPUT_COMMAND = 20,
		THREAD_STARTER_MESSAGE = 21,
		GUILD_INVITE_REMINDER = 22,
		CONTEXT_MENU_COMMAND = 23,
		AUTO_MODERATION_ACTION = 24,
		ROLE_SUBSCRIPTION_PURCHASE = 25,
		INTERACTION_PREMIUM_UPSELL = 26,
		STAGE_START = 27,
		STAGE_END = 28,
		STAGE_SPEAKER = 29,
		STAGE_RAISE_HAND = 30,
		STAGE_TOPIC = 31,
		GUILD_APPLICATION_PREMIUM_SUBSCRIPTION = 32,
		// PRIVATE_CHANNEL_INTEGRATION_ADDED = 33,
		// PRIVATE_CHANNEL_INTEGRATION_REMOVED = 34,
		PREMIUM_REFERRAL = 35,
		GUILD_INCIDENT_ALERT_MODE_ENABLED = 36,
		GUILD_INCIDENT_ALERT_MODE_DISABLED = 37,
		GUILD_INCIDENT_REPORT_RAID = 38,
		GUILD_INCIDENT_REPORT_FALSE_ALARM = 39,
		GUILD_DEADCHAT_REVIVE_PROMPT = 40,
		CUSTOM_GIFT = 41,
		GUILD_GAMING_STATS_PROMPT = 42,
		// POLL = 43,
		PURCHASE_NOTIFICATION = 44,
		// VOICE_HANGOUT_INVITE = 45,
		POLL_RESULT = 46,
		CHANGELOG = 47,
		NITRO_NOTIFICATION = 48,
		CHANNEL_LINKED_TO_LOBBY = 49,
		GIFTING_PROMPT = 50,
		IN_GAME_MESSAGE_NUX = 51,
		GUILD_JOIN_REQUEST_ACCEPT_NOTIFICATION = 52,
		GUILD_JOIN_REQUEST_REJECT_NOTIFICATION = 53,
		GUILD_JOIN_REQUEST_WITHDRAWN_NOTIFICATION = 54,
		HD_STREAMING_UPGRADED = 55,
		_ => Unknown(u8),
	}
}

bitflags! {
  pub struct MessageFlags: u64 {
	const CROSSPOSTED = 1 << 0;
	const IS_CROSSPOST = 1 << 1;
	const SUPPRESS_EMBEDS = 1 << 2;
	const SOURCE_MESSAGE_DELETED = 1 << 3;
	const URGENT = 1 << 4;
	const HAS_THREAD = 1 << 5;
	const EPHEMERAL = 1 << 6;
	const LOADING = 1 << 7;
	const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
	const GUILD_FEED_HIDDEN = 1 << 9;
	const SHOULD_SHOW_LINK_NOT_DISCORD_WARNING = 1 << 10;
	const SUPPRESS_NOTIFICATIONS = 1 << 12;
	const IS_VOICE_MESSAGE = 1 << 13;
	const HAS_SNAPSHOT = 1 << 14;
	const IS_COMPONENTS_V2 = 1 << 15;
		const SENT_BY_SOCIAL_LAYER_INTEGRATION = 1 << 16;
  }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageActivity {
	/// https://docs.discord.food/resources/presence#activity-action-type
	pub r#type:     u8,
	pub session_id: String,
	pub party_id:   String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageCall {
	pub participants:    Vec<Snowflake>,
	pub ended_timestamp: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageInteractionMetadata {
	pub id:                              Snowflake,
	/// https://docs.discord.food/interactions/receiving-and-responding#interaction-type
	pub r#type:                          u8,
	pub name:                            String,
	/// https://docs.discord.food/interactions/application-commands#application-command-types
	pub command_type:                    u8,
	/// https://docs.discord.food/resources/message#ephemerality-reason
	pub ephemerality_reason:             u8,
	pub user:                            User,
	pub authorizing_integration_owners:  HashMap<u32, Snowflake>,
	pub original_response_message_id:    Snowflake,
	pub interacted_message_id:           Snowflake,
	pub triggering_interaction_metadata: Box<MessageInteractionMetadata>,
	pub target_user:                     User,
	pub target_message_id:               Snowflake,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum MessageEphemeralityReason {
		NONE = 0,
		FEATURE_LIMITED = 1,
		GUILD_FEATURE_LIMITED = 2,
		USER_FEATURE_LIMITED = 3,
		SLOWMODE = 4,
		RATE_LIMIT = 5,
		CANNOT_MESSAGE_USER = 6,
		USER_VERIFICATION_LEVEL = 7,
		CANNOT_UNARCHIVE_THREAD = 8,
		CANNOT_JOIN_THREAD = 9,
		MISSING_PERMISSIONS = 10,
		CANNOT_SEND_ATTACHMENTS = 11,
		CANNOT_SEND_EMBEDS = 12,
		CANNOT_SEND_STICKERS = 13,
		AUTOMOD_BLOCKED = 14,
		HARMFUL_LINK = 15,
		CANNOT_USE_COMMAND = 16,
		BETA_GUILD_SIZE = 17,
		CANNOT_USE_EXTERNAL_APPS = 18,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageRoleSubscription {
	pub role_subscription_listing_id: Snowflake,
	pub tier_name:                    String,
	pub total_months_subscribed:      u16,
	pub is_renewal:                   bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessagePurchaseNotification {
	/// https://docs.discord.food/resources/message#message-purchase-notification-type
	pub r#type:                 u8,
	pub guild_product_purchase: Option<GuildProductPurchase>,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum MessagePurchaseNotificationType {
		GUILD_PRODUCT = 0,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageGuildProductPurchase {
	pub listing_id:   Snowflake,
	pub product_name: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageGiftInfo {
	pub emoji: Emoji,
	pub sound: SoundboardSound,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageSoundboardSound {
	pub id: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageReference {
	/// https://docs.discord.food/resources/message#message-reference-type
	pub r#type:             u8,
	pub message_id:         Snowflake,
	pub channel_id:         Snowflake,
	pub guild_id:           Snowflake,
	pub fail_if_not_exists: bool,
	pub forward_only:       ForwardOnly,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum MessageReferenceType {
		DEFAULT = 0,
		FORWARD = 1,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageForwardOnly {
	pub embed_indices:  Vec<u8>,
	pub attachment_ids: Vec<Snowflake>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageSnapshot {
	pub message: SnapshotMessage, // what the fuck is the point of this wrapper struct discord
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SnapshotMessage {
	pub content:           String,
	pub timestamp:         Timestamp,
	pub edited_timestamp:  Option<Timestamp>,
	pub mentions:          Vec<User>,
	pub mention_roles:     Vec<Snowflake>,
	pub attachments:       Vec<MessageAttachment>,
	pub embeds:            Vec<MessageEmbed>,
	/// https://docs.discord.food/resources/message#message-type
	pub r#type:            u8,
	/// https://docs.discord.food/resources/message#message-flags
	pub flags:             u64,
	pub components:        Vec<MessageComponent>,
	pub sticker_items:     Vec<StickerItem>,
	pub soundboard_sounds: Vec<SoundboardSound>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageReaction {
	pub count:         u16,
	pub count_details: ReactionCountDetails,
	pub me:            bool,
	pub me_burst:      bool,
	pub emoji:         Emoji,
	pub burst_colors:  Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageReactionCountDetails {
	pub normal: u16,
	pub burst:  u16,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum MessageReactionType {
		NORMAL = 0,
		BURST = 1,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageEmbed {
	pub title:                String,
	/// https://docs.discord.food/resources/message#embed-type
	pub r#type:               String,
	pub description:          String,
	pub url:                  String,
	pub timestamp:            Timestamp,
	pub color:                u32,
	pub footer:               EmbedFooter,
	pub image:                EmbedMedia,
	pub thumbnail:            EmbedMedia,
	pub video:                EmbedMedia,
	pub provider:             EmbedProvider,
	pub author:               EmbedAuthor,
	pub fields:               Vec<EmbedField>,
	pub reference_id:         Snowflake,
	pub content_scan_version: u8,
	/// https://docs.discord.food/resources/message#embed-flags
	pub flags:                u64,
}

pub enum EmbedType {
	application_news,
	article,
	auto_moderation_message,
	auto_moderation_notification,
	gift,
	gifv, // not a typo
	image,
	link,
	poll_result,
	post_preview,
	rich,
	video,
}

bitflags! {
  pub struct EmbedFlags: u64 {
	const CONTAINS_EXPLICIT_MEDIA = 1 << 4;
		const CONTENT_INVENTORY_ENTRY = 1 << 5;
  }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmbedMedia {
	pub url:                   String,
	pub proxy_url:             String,
	pub height:                u16,
	pub width:                 u16,
	/// https://docs.discord.food/resources/message#attachment-flags
	pub flags:                 u64,
	pub content_scan_metadata: String,
	pub placeholder_version:   u8,
	pub placeholder:           String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmbedProvider {
	pub name: String,
	pub url:  String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmbedAuthor {
	pub name:           String,
	pub url:            String,
	pub icon_url:       String,
	pub proxy_icon_url: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmbedFooter {
	pub text:           String,
	pub icon_url:       String,
	pub proxy_icon_url: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmbedField {
	pub name:   String,
	pub value:  String,
	pub inline: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ContentScanMetadata {
	/// https://docs.discord.food/resources/message#content-scan-flags
	pub flags:   u64,
	pub version: u8,
}

bitflags! {
  pub struct ContentScanFlags: u64 {
	const EXPLICIT = 1 << 0;
  }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MessageAttachment {
	pub id:                   Snowflake,
	pub filename:             String,
	pub title:                String,
	pub uploaded_filename:    String,
	pub description:          String,
	pub content_type:         String,
	pub size:                 u32,
	pub url:                  String,
	pub proxy_url:            String,
	pub height:               Option<u16>,
	pub width:                Option<u16>,
	pub content_scan_version: u8,
	pub placeholder_version:  u8,
	pub placeholder:          String,
	pub ephemeral:            bool,
	pub duration_secs:        f64,
	pub waveform:             String, /* Base64 encoded bytearray representing a sampled waveform (if voice message) */
	pub flags:                u64,    /* https://docs.discord.food/resources/message#attachment-flags */
	pub is_clip:              bool,
	pub is_thumbnail:         bool,
	pub is_remix:             bool,
	pub is_spoiler:           bool,
	pub clip_created_at:      Timestamp,
	pub clip_participant_ids: Vec<Snowflake>,
	pub clip_participants:    Vec<User>,
	pub application_id:       Snowflake,
	pub application:          Application,
}

bitflags! {
  pub struct AttachmentFlags: u64 {
	const IS_CLIP = 1 << 0;
	const IS_THUMBNAIL = 1 << 1;
	const IS_REMIX = 1 << 2;
	const IS_SPOILER = 1 << 3;
	const CONTAINS_EXPLICIT_MEDIA = 1 << 4;
		const IS_ANIMATED = 1 << 5;
  }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AllowedMentions {
	pub parse:        Vec<String>,
	pub roles:        Vec<Snowflake>,
	pub users:        Vec<Snowflake>,
	pub replied_user: bool,
}

pub enum AllowedMentionTypes {
	roles,
	users,
	everyone,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Poll {
	pub question:          PollMedia,
	pub answers:           Vec<PollAnswer>,
	pub expiry:            Option<Timestamp>,
	pub allow_multiselect: bool,
	/// https://docs.discord.food/resources/message#poll-layout-type
	pub layout_type:       u8,
	pub results:           PollResults,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PollCreate {
	pub question:          PollMedia,
	pub answers:           Vec<PollAnswer>,
	pub duraction:         u32,
	pub allow_multiselect: bool,
	/// https://docs.discord.food/resources/message#poll-layout-type
	pub layout_type:       u8,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum PollLayoutType {
		DEFAULT = 1,
		// IMAGE_ONLY_ANSWERS = 2,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PollMedia {
	pub text:  String,
	pub emoji: Emoji,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PollAnswer {
	pub answer_id:  u8,
	pub poll_media: PollMedia,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PollResults {
	pub is_finalized:  bool,
	pub answer_counts: Vec<PollAnswerCount>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PollAnswerCount {
	pub id:       u8,
	pub count:    u32,
	pub me_voted: bool,
}

// another one of those bs things that look like a pain to deserialize https://docs.discord.food/resources/message#example-poll-result-embed
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PollResultNotifications {
	pub poll_question_text:           String,
	pub total_votes:                  u32,
	pub victor_answer_id:             u8,
	pub victor_answer_text:           String,
	pub victor_answer_emoji_id:       Snowflake,
	pub victor_answer_emoji_name:     String,
	pub victor_answer_emoji_animated: bool,
	pub victor_answer_votes:          u32,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Potion {
	pub used_by:    Snowflake,
	pub r#type:     u8,
	pub emoji:      Vec<Emoji>,
	pub created_at: Timestamp,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum PotionType {
		CONFETTI = 0,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConfettiPotion {
	pub message_emoji: Emoji,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationSummary {
	pub id:          Snowflake,
	pub topic:       String,
	pub summ_short:  String,
	pub message_ids: Vec<Snowflake>,
	pub people:      Vec<Snowflake>,
	pub r#unsafe:    bool,
	pub start_id:    Snowflake,
	pub end_id:      Snowflake,
	pub count:       u16,
	/// https://docs.discord.food/resources/message#summary-source
	pub source:      u8,
	/// https://docs.discord.food/resources/message#summary-type
	pub r#type:      u8,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum SummarySource {
		SOURCE_0 = 0,
		SOURCE_1 = 1,
		SOURCE_2 = 2,
		_ => Unknown(u8),
	}
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum SummaryType {
		UNSET = 0,
		SOURCE_1 = 1,
		SOURCE_2 = 2,
		UNKNOWN = 3,
		_ => Unknown(u8),
	}
}
