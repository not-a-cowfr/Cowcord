#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::auto_moderation::AutomodRule;
use super::channel::Channel;
use super::guild_scheduled_event::GuildScheduledEvent;
use super::integration::Integration;
use super::user::User;
use super::webhook::Webhook;
use crate::models::types::Snowflake;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AuditLog {
	pub audit_log_entries:      Vec<LogEntry>,
	pub application_commands:   Vec<ApplicationCommand>,
	pub auto_moderation_rules:  Vec<AutomodRule>,
	pub guild_scheduled_events: Vec<GuildScheduledEvent>,
	pub integrations:           Vec<Integration>,
	pub threads:                Vec<Channel>,
	pub users:                  Vec<User>,
	pub webhooks:               Vec<Webhook>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LogEntry {
	pub target_id:   Option<String>,
	pub changes:     Vec<LogChange>,
	pub user_id:     Option<Snowflake>,
	pub id:          Snowflake,
	pub action_type: LogEvents,
	pub options:     Option<OptionalLogInfo>,
	pub reason:      String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LogEvents {
	GUILD_UPDATE = 1,
	CHANNEL_CREATE = 10,
	CHANNEL_UPDATE = 11,
	CHANNEL_DELETE = 12,
	CHANNEL_OVERWRITE_CREATE = 13,
	CHANNEL_OVERWRITE_UPDATE = 14,
	CHANNEL_OVERWRITE_DELETE = 15,
	MEMBER_KICK = 20,
	MEMBER_PRUNE = 21,
	MEMBER_BAN_ADD = 22,
	MEMBER_BAN_REMOVE = 23,
	MEMBER_UPDATE = 24,
	MEMBER_ROLE_UPDATE = 25,
	MEMBER_MOVE = 26,
	MEMBER_DISCONNECT = 27,
	BOT_ADD = 28,
	ROLE_CREATE = 30,
	ROLE_UPDATE = 31,
	ROLE_DELETE = 32,
	INVITE_CREATE = 40,
	INVITE_UPDATE = 41,
	INVITE_DELETE = 42,
	WEBHOOK_CREATE = 50,
	WEBHOOK_UPDATE = 51,
	WEBHOOK_DELETE = 52,
	EMOJI_CREATE = 60,
	EMOJI_UPDATE = 61,
	EMOJI_DELETE = 62,
	MESSAGE_DELETE = 72,
	MESSAGE_BULK_DELETE = 73,
	MESSAGE_PIN = 74,
	MESSAGE_UNPIN = 75,
	INTEGRATION_CREATE = 80,
	INTEGRATION_UPDATE = 81,
	INTEGRATION_DELETE = 82,
	STAGE_INSTANCE_CREATE = 83,
	STAGE_INSTANCE_UPDATE = 84,
	STAGE_INSTANCE_DELETE = 85,
	STICKER_CREATE = 90,
	STICKER_UPDATE = 91,
	STICKER_DELETE = 92,
	GUILD_SCHEDULED_EVENT_CREATE = 100,
	GUILD_SCHEDULED_EVENT_UPDATE = 101,
	GUILD_SCHEDULED_EVENT_DELETE = 102,
	THREAD_CREATE = 110,
	THREAD_UPDATE = 111,
	THREAD_DELETE = 112,
	APPLICATION_COMMAND_PERMISSION_UPDATE = 121,
	SOUNDBOARD_SOUND_CREATE = 130,
	SOUNDBOARD_SOUND_UPDATE = 131,
	SOUNDBOARD_SOUND_DELETE = 132,
	AUTO_MODERATION_RULE_CREATE = 140,
	AUTO_MODERATION_RULE_UPDATE = 141,
	AUTO_MODERATION_RULE_DELETE = 142,
	AUTO_MODERATION_BLOCK_MESSAGE = 143,
	AUTO_MODERATION_FLAG_TO_CHANNEL = 144,
	AUTO_MODERATION_USER_COMMUNICATION_DISABLED = 145,
	AUTO_MODERATION_QUARANTINE_USER = 146,
	CREATOR_MONETIZATION_REQUEST_CREATED = 150,
	CREATOR_MONETIZATION_TERMS_ACCEPTED = 151,
	ONBOARDING_PROMPT_CREATE = 163,
	ONBOARDING_PROMPT_UPDATE = 164,
	ONBOARDING_PROMPT_DELETE = 165,
	ONBOARDING_CREATE = 166,
	ONBOARDING_UPDATE = 167,
	// HARMFUL_LINKS_BLOCKED_MESSAGE = 180,
	VOICE_CHANNEL_STATUS_CREATE = 192,
	VOICE_CHANNEL_STATUS_DELETE = 193,
	// CLYDE_AI_PROFILE_UPDATE = 194,
	GUILD_SCHEDULED_EVENT_EXCEPTION_CREATE = 200,
	GUILD_SCHEDULED_EVENT_EXCEPTION_UPDATE = 201,
	GUILD_SCHEDULED_EVENT_EXCEPTION_DELETE = 202,
	GUILD_MEMBER_VERIFICATION_UPDATE = 210,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OptionalLogInfo {
	pub application_id:                    Snowflake,
	pub auto_moderation_rule_name:         String,
	pub auto_moderation_rule_trigger_type: AutomodTriggerType,
	pub channel_id:                        Snowflake,
	pub count:                             String,
	pub delete_member_days:                String,
	pub event_exception_id:                Snowflake,
	pub id:                                Snowflake,
	/// https://docs.discord.food/resources/integration#integration-type
	pub integration_type:                  String,
	pub members_removed:                   String,
	pub message_id:                        Snowflake,
	pub role_name:                         String,
	pub status:                            String,
	/// serialized as string but is u8 https://docs.discord.food/resources/channel#permission-overwrite-type
	pub r#type:                            String,
}

// theres a lot of changing to these types https://docs.discord.food/resources/audit-log#audit-log-change-exceptions
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LogChange {
	pub new_value: String,
	pub old_value: String,
	pub key:       String,
}
