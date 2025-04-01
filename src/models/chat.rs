use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use super::guild::GuildMember;
use super::types::Snowflake;
use super::user::{Nick, ThreadMember, User};

#[derive(Deserialize, Debug)]
pub struct MessageHistoryResponse {
	pub messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct MessageHistoryRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub around: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after:  Option<String>,
	pub limit:  u8,
}

#[derive(Deserialize, Debug)]
pub struct Channel {
	pub id:                                 Snowflake,
	pub r#type:                             u8, // https://docs.discord.sex/resources/channel#channel-type
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
	pub recipient_falgs:                    u8, // https://docs.discord.sex/resources/channel#recipient-flags
	pub icon:                               Option<String>, // TODO: specific types for cdn hashes https://docs.discord.sex/reference#cdn-formatting
	pub nicks:                              Vec<Nick>,
	pub managed:                            bool,
	pub blocked_user_warning_dismissed:     bool,
	pub safety_warnings:                    Vec<SafetyWarning>,
	pub application_id:                     Snowflake,
	pub owner_id:                           Snowflake,
	pub owner:                              Option<GuildMember>,
	pub parent_id:                          Option<Snowflake>,
	pub last_pin_timestamp:                 Option<Timestamp>,
	pub rtc_region:                         Option<String>, // https://docs.discord.sex/resources/voice#voice-region-object
	pub video_quality_mode:                 u8, // https://docs.discord.sex/resources/channel#video-quality-mode
	pub total_message_sent:                 usize, // like message_count except it counts deleted messages and intial message
	pub message_count:                      usize,
	pub member_count:                       u8, // stops counting at 50, nice one discord
	pub member_ids_preview:                 Vec<Snowflake>,
	pub thread_metadata:                    ThreadMetaData,
	pub member:                             ThreadMember,
	pub default_auto_archive_duration:      Option<u16>,
	pub default_thread_rate_limit_per_user: isize,
	pub permissions:                        String,
	pub flags:                              u8, // https://docs.discord.sex/resources/channel#channel-flags
	pub available_tags:                     Vec<Tag>, // max 5
	pub applied_tags:                       Vec<Snowflake>,
	pub default_reaction_emoji:             Option<DefaultReaction>,
	pub default_forum_layout:               u8, // https://docs.discord.sex/resources/channel#forum-layout-type
	pub default_sort_order:                 Option<u8>, // https://docs.discord.sex/resources/channel#sort-order-type
	pub icon_emoji:                         Option<IconEmoji>,
	pub is_message_request:                 bool,
	pub is_message_request_timestamp:       Option<Timestamp>,
	pub is_spam:                            bool,
	pub theme_color:                        Option<u32>,
	pub status:                             Option<String>, // max 500 characters
	pub hd_streaming_until:                 Option<Timestamp>,
	pub hd_streaming_buyer_id:              Option<Snowflake>,
}

#[derive(Deserialize, Debug)]
pub struct DefaultReaction {
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct IconEmoji {
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
	pub id:         Snowflake,
	pub name:       String,
	pub moderated:  bool,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ThreadMetaData {
	pub archived:              bool,
	pub auto_archive_duration: u16,
	pub archive_timestamp:     Timestamp,
	pub locked:                bool,
	pub invitable:             bool,
	pub create_timestamp:      Option<Timestamp>,
}

#[derive(Deserialize, Debug)]
pub struct PermissionOverwrite {
	pub id:     Snowflake,
	pub r#type: u8, // https://docs.discord.sex/resources/channel#permission-overwrite-type
	pub allow:  String, // TODO: smth to convert bitwisevalue of perms into a hash of perms
	pub deny:   String,
}

#[derive(Deserialize, Debug)]
pub struct SafetyWarning {
	pub id:                String,
	pub r#type:            u8, // https://docs.discord.sex/resources/channel#safety-warning-type
	pub expiry:            Timestamp,
	pub dismiss_timestamp: Option<Timestamp>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
	pub id:               Snowflake,
	pub channel_id:       Snowflake,
	pub author:           User,
	pub content:          String,
	pub timestamp:        Timestamp,
	pub edited_timestamp: Option<Timestamp>,
	pub tts:              bool,
	pub mention_everyone: bool,
	pub mentions:         Vec<User>,
	pub mention_channels: Vec<Channel>,
}
