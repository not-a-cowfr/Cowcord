use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use super::channel::Channel;
use super::directory::GuildScheduledEvent;
use super::emoji::Emoji;
use super::guild::{Guild, GuildMember, Role};
use super::presence::Presence;
use super::stage_instance::StageInstance;
use super::sticker::Sticker;
use super::voice::VoiceState;

#[derive(Deserialize)]
#[serde(default)]
pub struct GatewayGuild {
	pub joined_at:                  Timestamp,
	pub large:                      bool,
	pub unavailable:                bool,
	pub geo_restricted:             bool,
	pub member_count:               u32,
	pub voice_states:               Vec<VoiceState>,
	pub members:                    Vec<GuildMember>,
	pub channels:                   Vec<Channel>,
	pub threads:                    Vec<Channel>,
	pub presences:                  Vec<Presence>,
	pub stage_instances:            Vec<StageInstance>,
	pub guild_scheduled_events:     Vec<GuildScheduledEvent>,
	/// https://docs.discord.sex/topics/gateway-events#data-mode
	pub data_mode:                  String,
	pub properties:                 Guild,
	pub stickers:                   Vec<Sticker>,
	pub roles:                      Vec<Role>,
	pub emojis:                     Vec<Emoji>,
	pub premium_subscription_count: u32,
}
