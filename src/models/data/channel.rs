#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use super::guild::GuildMember;
use super::user::User;
use super::user_settings::MuteConfig;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Channel {
	pub id:                                 Snowflake,
	/// https://docs.discord.sex/resources/channel#channel-type
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
	/// https://docs.discord.sex/resources/channel#recipient-flags
	pub recipient_falgs:                    u8,
	pub icon:                               Option<String>, // TODO: specific types for cdn hashes https://docs.discord.sex/reference#cdn-formatting
	pub nicks:                              Vec<ChannelNick>,
	pub managed:                            bool,
	pub blocked_user_warning_dismissed:     bool,
	pub safety_warnings:                    Vec<SafetyWarning>,
	pub application_id:                     Snowflake,
	pub owner_id:                           Snowflake,
	pub owner:                              Option<GuildMember>,
	pub parent_id:                          Option<Snowflake>,
	pub last_pin_timestamp:                 Option<Timestamp>,
	/// https://docs.discord.sex/resources/voice#voice-region-object
	pub rtc_region:                         Option<String>,
	/// https://docs.discord.sex/resources/channel#video-quality-mode
	pub video_quality_mode:                 u8,
	pub total_message_sent:                 usize, // like message_count except it counts deleted messages and intial thread message
	pub message_count:                      usize,
	pub member_count:                       u8, // stops counting at 50, nice one discord
	pub member_ids_preview:                 Vec<Snowflake>,
	pub thread_metadata:                    ThreadMetaData,
	pub member:                             ThreadMember,
	pub default_auto_archive_duration:      Option<u16>,
	pub default_thread_rate_limit_per_user: isize,
	pub permissions:                        String,
	/// https://docs.discord.sex/resources/channel#channel-flags
	pub flags:                              u8,
	pub available_tags:                     Vec<ForumTag>, // max 5
	pub applied_tags:                       Vec<Snowflake>,
	pub default_reaction_emoji:             Option<DefaultReaction>,
	/// https://docs.discord.sex/resources/channel#forum-layout-type
	pub default_forum_layout:               u8,
	/// https://docs.discord.sex/resources/channel#sort-order-type
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

// Obfuscated channel names and topics are always returned as ___hidden___
pub enum ChannelType {
	___hidden___,
	GUILD_TEXT,
	DM,
	GUILD_VOICE,
	GROUP_DM,
	GUILD_CATEGORY,
	GUILD_NEWS,
	GUILD_STORE,
	// GUILD_LFG,
	// LFG_GROUP_DM,
	// THREAD_ALPHA,
	NEWS_THREAD,
	PUBLIC_THREAD,
	PRIVATE_THREAD,
	GUILD_STAGE_VOICE,
	GUILD_DIRECTORY,
	GUILD_FORUM,
	GUILD_MEDIA,
	LOBBY,
	EPHEMERAL_DM,
}

pub enum RecipientFlags {
	DISMISSED_IN_GAME_MESSAGE_NUX = 1 << 0,
}

pub enum VideoQualityMode {
	AUTO,
	FULL,
}

pub enum ChannelFlag {
	GUILD_FEED_REMOVED = 1 << 0,
	PINNED = 1 << 1,
	ACTIVE_CHANNELS_REMOVED = 1 << 2,
	REQUIRE_TAG = 1 << 4,
	IS_SPAM = 1 << 5,
	IS_GUILD_RESOURCE_CHANNEL = 1 << 7,
	CLYDE_AI = 1 << 8,
	IS_SCHEDULED_FOR_DELETION = 1 << 9,
	// IS_MEDIA_CHANNEL = 1 << 10,
	SUMMARIES_DISABLED = 1 << 11,
	// APPLICATION_SHELF_CONSENT = 1 << 12,
	IS_ROLE_SUBSCRIPTION_TEMPLATE_PREVIEW_CHANNEL = 1 << 13,
	IS_BROADCASTING = 1 << 14,
	HIDE_MEDIA_DOWNLOAD_OPTIONS = 1 << 15,
	IS_JOIN_REQUEST_INTERVIEW_CHANNEL = 1 << 16,
	OBFUSCATED = 1 << 17,
}

pub enum FormLayoutType {
	DEFAULT,
	LIST,
	GRID,
}

pub enum SortOrderType {
	LATEST_ACTIVITY,
	CREATION_TIME,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ChannelNick {
	pub id:   Snowflake,
	pub nick: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct SafetyWarning {
	pub id:                String,
	/// https://docs.discord.sex/resources/channel#safety-warning-type
	pub r#type:            u8,
	pub expiry:            Timestamp,
	pub dismiss_timestamp: Option<Timestamp>,
}

pub enum SafetWarningType {
	STRANGER_DANGER,
	INAPPROPRIATE_CONVERSATION_TIER_1,
	INAPPROPRIATE_CONVERSATION_TIER_2,
	LIKELY_ATO,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct FollowedChannel {
	pub channel_id: Snowflake,
	pub webhook_id: Snowflake,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PermissionOverwrite {
	pub id:     Snowflake,
	/// https://docs.discord.sex/resources/channel#permission-overwrite-type
	pub r#type: u8,
	pub allow:  String, // TODO: smth to convert bitwisevalue of perms into a hash of perms
	pub deny:   String,
}

pub enum PermissionOverwriteType {
	role = 0,
	member = 1,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ThreadMetaData {
	pub archived:              bool,
	pub auto_archive_duration: u16,
	pub archive_timestamp:     Timestamp,
	pub locked:                bool,
	pub invitable:             bool,
	pub create_timestamp:      Option<Timestamp>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ThreadMember {
	pub id:             Snowflake,
	pub user_id:        Snowflake,
	pub join_timestamp: Timestamp,
	/// https://docs.discord.sex/resources/channel#thread-member-flags
	pub flags:          u8,
	pub muted:          bool,
	pub mute_config:    MuteConfig,
	pub member:         GuildMember,
}

pub enum ThreadMemberFlags {
	HAS_INTERACTED = 1 << 0,
	ALL_MESSAGES = 1 << 1,
	ONLY_MENTIONS = 1 << 2,
	NO_MESSAGES = 1 << 3,
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
