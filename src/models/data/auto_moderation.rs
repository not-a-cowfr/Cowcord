#![allow(non_camel_case_types)]

use std::collections::HashMap;

use super::types::Timestamp;
use serde::Deserialize;

use super::types::Snowflake;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodAlert {
	pub rule_name:                    String,
	pub decision_id:                  String,
	pub decision_reason:              String,
	/// https://docs.discord.sex/resources/auto-moderation#automod-decision-outcome
	pub decision_outcome:             String,
	pub channel_id:                   Snowflake,
	pub flagged_message_id:           Snowflake,
	pub keyword:                      String,
	pub keyword_matched_content:      String,
	/// https://docs.discord.sex/resources/auto-moderation#automod-profile-update-type
	pub block_profile_update_type:    String,
	/// https://docs.discord.sex/resources/auto-moderation#automod-quarantine-user-reason
	pub quarantine_user:              String,
	/// https://docs.discord.sex/resources/auto-moderation#automod-quarantine-user-action
	pub quarantine_user_action:       String,
	/// https://docs.discord.sex/resources/auto-moderation#automod-quarantine-event-type
	pub quarantine_event:             String,
	/// https://docs.discord.sex/resources/auto-moderation#automod-decision-outcome
	pub voice_channel_status_outcome: String,
	pub application_name:             String,
	pub interaction_user_id:          Snowflake,
	/// https://docs.discord.sex/resources/auto-moderation#automod-interaction-callback-type
	pub interaction_callback_type:    String,
	pub timeout_duration:             u32,
	pub alert_actions_execution:      AlertActionsExecution,
}

pub enum DecisionOutcome {
	flagged,
	blocked,
}

pub enum ProfileUpdateType {
	nickname_update,
	nickname_reset,
}

pub enum QuarantineUserReason {
	username,
	display_name,
	// bio,
	nickname,
	clan_tag,
}

pub enum QuarantineUserAction {
	block_guest_join,
	block_profile_update,
	quarantine_user,
}

pub enum QuarantineEventType {
	guild_join,
	message_send,
	username_update,
	clan_tag_update,
}

pub enum InteractionCallbackType {
	modal,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodAlertActionsExecution {
	pub v:       u8,
	pub actions: HashMap<String, AutomodAlertAction>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodAlertAction {
	pub actor: Snowflake,
	pub ts:    Timestamp, // pmo
}

pub enum AutomodAlertActionType {
	SET_COMPLETED,
	UNSET_COMPLETED,
	DELETE_USER_MESSAGE,
	SUBMIT_FEEDBACK,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodIncidentNotificationEmbed {
	/// https://docs.discord.sex/resources/auto-moderation#automod-incident-notification-type
	pub notification_type:                 String,
	pub decision_id:                       String,
	pub action_by_user_id:                 Snowflake,
	/// https://docs.discord.sex/resources/auto-moderation#automod-raid-type
	pub raid_type:                         String,
	pub raid_datetime:                     Timestamp,
	pub join_attempts:                     u32,
	pub dms_sent:                          u32,
	pub suspicious_mention_activity_until: Timestamp,
	/// https://docs.discord.sex/resources/auto-moderation#automod-raid-resolution-reason
	pub resolved_reason:                   String,
}

pub enum AutomodIncidentNotificationType {
	activity_alerts_enabled,
	raid,
	mention_raid,
}

pub enum RaidType {
	JOIN_RAID,
	MENTION_RAID,
}

pub enum RaidResolutonReason {
	LEGITIMATE_ACTIVITY,
	LEGITIMATE_ACCOUNTS,
	LEGITIMATE_DMS,
	DM_SPAM,
	JOIN_RAID,
	OTHER,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodRules {
	pub id:               Snowflake,
	pub guild_id:         Snowflake,
	pub name:             String,
	pub creator_id:       Snowflake,
	/// https://docs.discord.sex/resources/auto-moderation#automod-event-type
	pub event_type:       u8,
	/// https://docs.discord.sex/resources/auto-moderation#automod-trigger-type
	pub trigger_type:     u8,
	pub trigger_metadata: AutomodTriggerMetadata,
	pub actions:          Vec<AutomodAction>,
	pub enabled:          bool,
	pub exempt_roles:     Vec<Snowflake>,
	pub exempt_channels:  Vec<Snowflake>,
}

pub enum AutomodEventType {
	MESSAGE_SEND = 1,
	GUILD_MEMBER_EVENT = 2,
}

pub enum AutomodTriggerType {
	KEYWORD = 1,
	HARMFUL_LINK = 2,
	SPAM = 3,
	KEYWORD_PRESET = 4,
	MENTION_SPAM = 5,
	USER_PROFILE = 6,
	GUILD_POLICY = 7,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodTriggerMetadata {
	pub keyword_filter:                  Vec<String>,
	pub regex_patterns:                  Vec<String>,
	/// https://docs.discord.sex/resources/auto-moderation#automod-keyword-preset-type
	pub presets:                         Vec<u8>,
	pub allow_list:                      Vec<String>,
	pub mention_total_limit:             u8,
	pub mention_raid_protection_enabled: bool,
}

pub enum AutomodKeywordPresetType {
	PROFANITY = 1,
	SEXUAL_CONTENT = 2,
	SLURS = 3,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodAction {
	/// https://docs.discord.sex/resources/auto-moderation#automod-action-type
	pub r#type:   u8,
	pub metadata: AutomodActionMetadata,
}

pub enum AutomodActionType {
	BLOCK_MESSAGE = 1,
	SEND_ALERT_MESSAGE = 2,
	TIMEOUT_USER = 3,
	QUARANTINE_USER = 4,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodActionMetadata {
	pub channel_id:       Snowflake,
	pub duration_seconds: u32,
	pub custom_message:   String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AutomodIncedentsData {
	pub raid_detected_at:       Option<Timestamp>,
	pub dm_spam_detected_at:    Option<Timestamp>,
	pub invites_disabled_until: Option<Timestamp>,
	pub dms_disabled_until:     Option<Timestamp>,
}
