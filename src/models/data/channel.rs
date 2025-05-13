#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::guild::GuildMember;
use super::message::{AllowedMentions, Message, MessageActivity, MessageAttachment, MessageEmbed};
use super::user::User;
use super::user_settings::MuteConfig;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Channel {
	pub id:                                 Snowflake,
	/// https://docs.discord.food/resources/channel#channel-type
	pub r#type:                             u8,
	pub guild_id:                           Snowflake,
	pub position:                           u16,
	pub permission_overwrites:              Vec<PermissionOverwrite>,
	pub names:                              Option<String>,
	pub topic:                              Option<String>,
	pub nsfw:                               bool,
	pub last_message_id:                    Option<Snowflake>,
	pub bitrate:                            u32,
	pub user_limit:                         u8,
	pub rate_limit_per_user:                u16,
	pub recipients:                         Vec<User>,
	/// https://docs.discord.food/resources/channel#recipient-flags
	pub recipient_falgs:                    u8,
	pub icon:                               Option<String>, /* TODO: specific types for cdn hashes https://docs.discord.food/reference#cdn-formatting */
	pub nicks:                              Vec<ChannelNick>,
	pub managed:                            bool,
	pub blocked_user_warning_dismissed:     bool,
	pub safety_warnings:                    Vec<SafetyWarning>,
	pub application_id:                     Snowflake,
	pub owner_id:                           Snowflake,
	pub owner:                              Option<GuildMember>,
	pub parent_id:                          Option<Snowflake>,
	pub last_pin_timestamp:                 Option<Timestamp>,
	/// https://docs.discord.food/resources/voice#voice-region-object
	pub rtc_region:                         Option<String>,
	/// https://docs.discord.food/resources/channel#video-quality-mode
	pub video_quality_mode:                 u8,
	pub total_message_sent:                 usize, /* like message_count except it counts deleted messages and intial thread message */
	pub message_count:                      usize,
	pub member_count:                       u8, // stops counting at 50, nice one discord
	pub member_ids_preview:                 Vec<Snowflake>,
	pub thread_metadata:                    ThreadMetaData,
	pub member:                             ThreadMember,
	pub default_auto_archive_duration:      Option<u16>,
	pub default_thread_rate_limit_per_user: isize,
	pub permissions:                        String,
	/// https://docs.discord.food/resources/channel#channel-flags
	pub flags:                              u64,
	pub available_tags:                     Vec<ForumTag>, // max 5
	pub applied_tags:                       Vec<Snowflake>,
	pub default_reaction_emoji:             Option<DefaultReaction>,
	/// https://docs.discord.food/resources/channel#forum-layout-type
	pub default_forum_layout:               u8,
	/// https://docs.discord.food/resources/channel#sort-order-type
	pub default_sort_order:                 Option<u8>,
	pub icon_emoji:                         Option<IconEmoji>,
	pub is_message_request:                 bool,
	pub is_message_request_timestamp:       Option<Timestamp>,
	pub is_spam:                            bool,
	pub theme_color:                        Option<u32>,
	pub status:                             Option<String>,
	pub hd_streaming_until:                 Option<Timestamp>,
	pub hd_streaming_buyer_id:              Option<Snowflake>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChannelType {
	GUILD_TEXT = 0,
	DM = 1,
	GUILD_VOICE = 2,
	GROUP_DM = 3,
	GUILD_CATEGORY = 4,
	GUILD_NEWS = 5,
	GUILD_STORE = 6,
	// GUILD_LFG = 7,
	// LFG_GROUP_DM = 8,
	// THREAD_ALPHA = 9,
	NEWS_THREAD = 10,
	PUBLIC_THREAD = 11,
	PRIVATE_THREAD = 12,
	GUILD_STAGE_VOICE = 13,
	GUILD_DIRECTORY = 14,
	GUILD_FORUM = 15,
	GUILD_MEDIA = 16,
	LOBBY = 17,
	EPHEMERAL_DM = 18,
}

bitflags! {
  pub struct RecipientFlags: u64 {
	const DISMISSED_IN_GAME_MESSAGE_NUX = 1 << 0;
  }
}
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VideoQualityMode {
	AUTO = 0,
	FULL = 1,
}

bitflags! {
  pub struct ChannelFlags: u64 {
	const GUILD_FEED_REMOVED = 1 << 0;
	const PINNED = 1 << 1;
	const ACTIVE_CHANNELS_REMOVED = 1 << 2;
	const REQUIRE_TAG = 1 << 4;
	const IS_SPAM = 1 << 5;
	const IS_GUILD_RESOURCE_CHANNEL = 1 << 7;
	const CLYDE_AI = 1 << 8;
	const IS_SCHEDULED_FOR_DELETION = 1 << 9;
	// const IS_MEDIA_CHANNEL = 1 << 10;
	const SUMMARIES_DISABLED = 1 << 11;
	// const APPLICATION_SHELF_CONSENT = 1 << 12;
	const IS_ROLE_SUBSCRIPTION_TEMPLATE_PREVIEW_CHANNEL = 1 << 13;
	const IS_BROADCASTING = 1 << 14;
	const HIDE_MEDIA_DOWNLOAD_OPTIONS = 1 << 15;
	const IS_JOIN_REQUEST_INTERVIEW_CHANNEL = 1 << 16;
		const OBFUSCATED = 1 << 17;
  }
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FormLayoutType {
	DEFAULT = 0,
	LIST = 1,
	GRID = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SortOrderType {
	LATEST_ACTIVITY = 0,
	CREATION_TIME = 1,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChannelNick {
	pub id:   Snowflake,
	pub nick: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SafetyWarning {
	pub id:                String,
	/// https://docs.discord.food/resources/channel#safety-warning-type
	pub r#type:            u8,
	pub expiry:            Timestamp,
	pub dismiss_timestamp: Option<Timestamp>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SafetWarningType {
	STRANGER_DANGER = 1,
	INAPPROPRIATE_CONVERSATION_TIER_1 = 2,
	INAPPROPRIATE_CONVERSATION_TIER_2 = 3,
	LIKELY_ATO = 4,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FollowedChannel {
	pub channel_id: Snowflake,
	pub webhook_id: Snowflake,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PermissionOverwrite {
	pub id:     Snowflake,
	pub r#type: PermissionOverwriteType,
	pub allow:  String, // TODO: smth to convert bitwise value of perms into a hash of perms
	pub deny:   String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PermissionOverwriteType {
	role = 0,
	member = 1,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadMetaData {
	pub archived:              bool,
	pub auto_archive_duration: u16,
	pub archive_timestamp:     Timestamp,
	pub locked:                bool,
	pub invitable:             bool,
	pub create_timestamp:      Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadMember {
	pub id:             Snowflake,
	pub user_id:        Snowflake,
	pub join_timestamp: Timestamp,
	/// https://docs.discord.food/resources/channel#thread-member-flags
	pub flags:          u64,
	pub muted:          bool,
	pub mute_config:    MuteConfig,
	pub member:         GuildMember,
}

bitflags! {
  pub struct ThreadMemberFlags: u64 {
	const HAS_INTERACTED = 1 << 0;
	const ALL_MESSAGES = 1 << 1;
	const ONLY_MENTIONS = 1 << 2;
		const NO_MESSAGES = 1 << 3;
  }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DefaultReaction {
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IconEmoji {
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumTag {
	pub id:         Snowflake,
	pub name:       String,
	pub moderated:  bool,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ConsentStatus {
	UNSPECIFIED = 0,
	PENDING = 1,
	ACCEPTED = 2,
	REJECTED = 3,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SupplementalMessageRequest {
	pub channel_id:      Snowflake,
	pub message_preview: Message,
}

pub enum ThreadSortType {
	last_message_time,
	archive_time,
	relevance,
	creation_time,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
/// you must provide a value for at least one of content, embeds, components, sticker_ids, activity, or files[n].
pub struct ThreadOnlyChannelMessageParams {
	content:          String,
	embeds:           Vec<MessageEmbed>,
	/// cannot be used by user accounts
	#[deprecated]
	embed:            MessageEmbed,
	allowed_mentions: AllowedMentions,
	/// cannot be used by user accounts
	components:       Vec<MessageComponent>,
	sticker_ids:      Vec<Snowflake>,
	activity:         MessageActivity,
	application_id:   Snowflake,
	/// https://docs.discord.food/resources/message#message-flags
	flags:            u64,
	// files[n]: file contents
	payload_json:     String,
	attachments:      Vec<MessageAttachment>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadPostData {
	pub owner:         Option<GuildMember>,
	pub first_message: Option<Message>,
}
