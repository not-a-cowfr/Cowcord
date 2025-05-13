#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodAlert {
	pub rule_name:                    String,
	pub decision_id:                  String,
	pub decision_reason:              String,
	pub decision_outcome:             DecisionOutcome,
	pub channel_id:                   Snowflake,
	pub flagged_message_id:           Snowflake,
	pub keyword:                      String,
	pub keyword_matched_content:      String,
	pub block_profile_update_type:    ProfileUpdateType,
	pub quarantine_user:              QuarantineUserReason,
	pub quarantine_user_action:       QuarantineUserAction,
	pub quarantine_event:             QuarantineEventType,
	pub voice_channel_status_outcome: DecisionOutcome,
	pub application_name:             String,
	pub interaction_user_id:          Snowflake,
	pub interaction_callback_type:    InteractionCallbackType,
	pub timeout_duration:             u32,
	pub alert_actions_execution:      AutomodAlertActionsExecution,
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

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodAlertActionsExecution {
	pub v:       u8,
	pub actions: HashMap<String, AutomodAlertAction>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodAlertAction {
	pub actor: Snowflake,
	pub ts:    Timestamp,
}

pub enum AutomodAlertActionType {
	SET_COMPLETED,
	UNSET_COMPLETED,
	DELETE_USER_MESSAGE,
	SUBMIT_FEEDBACK,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodIncidentNotificationEmbed {
	pub notification_type:                 AutomodIncidentNotificationType,
	pub decision_id:                       String,
	pub action_by_user_id:                 Snowflake,
	pub raid_type:                         RaidType,
	pub raid_datetime:                     Timestamp,
	pub join_attempts:                     u32,
	pub dms_sent:                          u32,
	pub suspicious_mention_activity_until: Timestamp,
	pub resolved_reason:                   RaidResolutonReason,
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

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodRule {
	pub id:               Snowflake,
	pub guild_id:         Snowflake,
	pub name:             String,
	pub creator_id:       Snowflake,
	pub event_type:       AutomodTriggerType,
	pub trigger_type:     AutomodTriggerType,
	pub trigger_metadata: AutomodTriggerMetadata,
	pub actions:          Vec<AutomodAction>,
	pub enabled:          bool,
	pub exempt_roles:     Vec<Snowflake>,
	pub exempt_channels:  Vec<Snowflake>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodEventType {
	MESSAGE_SEND = 1,
	GUILD_MEMBER_EVENT = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodTriggerType {
	KEYWORD = 1,
	HARMFUL_LINK = 2,
	SPAM = 3,
	KEYWORD_PRESET = 4,
	MENTION_SPAM = 5,
	USER_PROFILE = 6,
	GUILD_POLICY = 7,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodTriggerMetadata {
	pub keyword_filter:                  Vec<String>,
	pub regex_patterns:                  Vec<String>,
	pub presets:                         Vec<AutomodKeywordPresetType>,
	pub allow_list:                      Vec<String>,
	pub mention_total_limit:             u8,
	pub mention_raid_protection_enabled: bool,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodKeywordPresetType {
	PROFANITY = 1,
	SEXUAL_CONTENT = 2,
	SLURS = 3,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodAction {
	pub r#type:   AutomodActionType,
	pub metadata: AutomodActionMetadata,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodActionType {
	BLOCK_MESSAGE = 1,
	SEND_ALERT_MESSAGE = 2,
	TIMEOUT_USER = 3,
	QUARANTINE_USER = 4,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodActionMetadata {
	pub channel_id:       Snowflake,
	pub duration_seconds: u32,
	pub custom_message:   String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutomodIncedentsData {
	pub raid_detected_at:       Option<Timestamp>,
	pub dm_spam_detected_at:    Option<Timestamp>,
	pub invites_disabled_until: Option<Timestamp>,
	pub dms_disabled_until:     Option<Timestamp>,
}
