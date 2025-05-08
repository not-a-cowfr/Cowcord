use serde::{Deserialize, Serialize};

use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserGuildSettingsArr {
	pub entries: Vec<UserGuildSettings>,
	pub partial: bool,
	pub version: u32,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationSettings {
	/// https://docs.discord.food/resources/user-settings#notification-settings-flags
	pub flags: u64,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserGuildSettings {
	pub channel_overrides:     ChannelOverrides,
	/// https://docs.discord.food/resources/user-settings#user-guild-settings-flags
	pub flags:                 u64,
	pub guild_id:              Option<Snowflake>,
	pub hide_muted_channels:   bool,
	/// https://docs.discord.food/resources/guild#message-notification-level
	pub message_notifications: u8,
	pub mobile_push:           bool,
	pub mute_scheduled_events: bool,
	pub muted:                 bool,
	pub mute_config:           Option<MuteConfig>,
	/// https://docs.discord.food/resources/user-settings#highlight-level
	pub notify_highlights:     u8,
	pub suppress_everyone:     u8,
	pub suppress_roles:        u8,
	pub version:               u16,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChannelOverrides {
	pub channel_id:            Snowflake,
	pub collapsed:             bool,
	/// https://docs.discord.food/resources/user-settings#channel-override-flags
	pub flags:                 u64,
	/// https://docs.discord.food/resources/guild#message-notification-level
	pub message_notifications: u16,
	pub muted:                 bool,
	pub mute_config:           Option<MuteConfig>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MuteConfig {
	pub end_time:             Option<Timestamp>,
	pub selected_time_window: isize,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserSettings {
	pub activity_restricted_guild_ids:              Vec<Snowflake>,
	pub activity_joining_restricted_guild_ids:      Vec<Snowflake>,
	pub afk_timeout:                                u32,
	pub allow_accessibility_detection:              bool,
	pub allow_activity_party_privacy_friends:       bool,
	pub allow_activity_party_privacy_voice_channel: bool,
	pub animate_emoji:                              bool,
	/// https://docs.discord.food/resources/user-settings#sticker-animation-option
	pub animate_stickers:                           u8,
	pub contact_sync_enabled:                       bool,
	pub convert_emoticons:                          bool,
	pub custom_status:                              Option<CustomStatus>,
	pub default_guilds_restricted:                  bool,
	pub detect_platform_accounts:                   bool,
	pub developer_mode:                             bool,
	pub disable_games_tab:                          bool,
	pub enable_tts_command:                         bool,
	pub explicit_content_filter:                    u8,
	pub friend_discovery_flags:                     u64,
	pub friend_source_flags:                        Option<FriendSourceFlags>,
	pub gif_auto_play:                              bool,
	pub guild_folders:                              Vec<GuildFolder>,
	pub inline_attachment_media:                    bool,
	pub inline_embed_media:                         bool,
	pub locale:                                     String,
	pub message_display_compact:                    bool,
	pub native_phone_integration_enabled:           bool,
	#[deprecated]
	pub passwordless:                               bool,
	pub render_embeds:                              bool,
	pub render_reactions:                           bool,
	pub restricted_guilds:                          Vec<Snowflake>,
	pub show_current_game:                          bool,
	pub status:                                     String,
	pub stream_notifications_enabled:               bool,
	pub theme:                                      String,
	pub timezone_offset:                            i16,
	pub view_nsfw_commands:                         bool,
	pub view_nsfw_guilds:                           bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GuildFolder {
	pub color:     Option<u32>,
	pub guild_ids: Vec<Snowflake>,
	pub id:        Option<u32>,
	pub name:      Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FriendSourceFlags {
	pub all:            bool,
	pub mutual_friends: bool,
	pub mutual_guilds:  bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomStatus {
	pub text:       Option<String>,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
	pub expires_at: Option<Timestamp>,
}
