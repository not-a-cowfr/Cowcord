use std::collections::HashMap;

use iso8601_timestamp::Timestamp;
use serde::Deserialize;

use super::chat::{ChannelSettings, CustomNotifSoundConfig};
use super::types::Snowflake;
use super::user::user::{AvatarDecorationData, User};

#[derive(Deserialize, Debug)]
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
	pub flags:                        u8, // https://docs.discord.sex/resources/guild#guild-member-flags
	pub permissions:                  String,
}

#[derive(Deserialize)]
pub struct AllGuildSettings {
	pub guild: HashMap<u64, GuildSettings>,
}

#[derive(Deserialize)]
pub struct GuildSettings {
	pub channels:                              HashMap<u64, ChannelSettings>,
	pub hub_progress:                          u32,
	pub guild_onboarding_progress:             u32,
	pub guild_recents_dismissed_at:            Timestamp,
	pub dismissed_guild_content:               Vec<u8>,
	pub join_sound:                            CustomCallSound,
	pub mobile_redesign_channel_list_settings: ChannelListSettings,
	pub disable_raid_alert_push:               bool,
	pub disable_raid_alert_nag:                bool,
	pub custom_notification_sound_config:      CustomNotifSoundConfig,
	pub leaderboards_disabled:                 bool,
}

#[derive(Deserialize)]
pub struct CustomCallSound {
	pub sound_id: u64,
	pub guild_id: u64,
}

#[derive(Deserialize)]
pub struct ChannelListSettings {
	pub layout:           Option<String>,
	pub message_previews: Option<String>,
}
