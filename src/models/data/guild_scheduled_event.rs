#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::guild::GuildMember;
use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GuildScheduledEvent {
	pub id:                               Snowflake,
	pub guild_id:                         Snowflake,
	pub channel_id:                       Option<Snowflake>,
	pub creator_id:                       Option<Snowflake>,
	pub creator:                          User,
	pub name:                             String,
	pub description:                      Option<String>,
	pub scheduled_start_time:             Timestamp,
	pub scheduled_end_time:               Option<Timestamp>,
	pub auto_start:                       bool,
	pub privacy_level:                    PrivacyLevel,
	pub status:                           GuildScheduledEventStatus,
	pub entity_type:                      GuildScheduledEventEntityType,
	pub entity_id:                        Option<Snowflake>,
	pub entity_metadata:                  Option<EntityMetadata>,
	pub user_count:                       u32,
	pub image:                            Option<String>,
	pub recurrence_rule:                  Option<GuildScheduledEventRecurrenceRule>,
	pub guild_scheduled_event_exceptions: Vec<GuildScheduledEventException>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventStatus {
	SCHEDULED = 1,
	ACTIVE = 2,
	COMPLETED = 3,
	CANCELED = 4,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventEntityType {
	STAGE_INSTANCE = 1,
	VOICE = 2,
	EXTERNAL = 3,
	PRIME_TIME = 4,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EntityMetadata {
	pub location: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GuildScheduledEventRecurrenceRule {
	pub start:        Timestamp,
	pub end:          Option<Timestamp>,
	pub frequency:    GuildScheduledEventRecurrenceRuleFrequency,
	/// The spacing between the events, defined by frequency; for example, frequency of WEEKLY and an interval of 2 would be every other week
	pub interval:     u16,
	pub by_weekday:   Option<Vec<u8>>,
	pub by_n_weekday: Option<Vec<NWeekday>>,
	pub by_month:     Option<Vec<u8>>,
	pub by_month_day: Option<Vec<u8>>,
	pub by_year_day:  Option<Vec<u16>>,
	pub count:        Option<u32>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleFrequency {
	YEARLY = 0,
	MONTHLY = 1,
	WEEKLY = 2,
	DAILY = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleWeekday {
	MONDAY = 0,
	TUESDAY = 1,
	WEDNESDAY = 2,
	THURSDAY = 3,
	FRIDAY = 4,
	SATURDAY = 5,
	SUNDAY = 6,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NWeekday {
	pub n:   u8,
	pub day: u8,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleMonth {
	JANUARY = 1,
	FEBRUARY = 2,
	MARCH = 3,
	APRIL = 4,
	MAY = 5,
	JUNE = 6,
	JULY = 7,
	AUGUST = 8,
	SEPTEMBER = 9,
	OCTOBER = 10,
	NOVEMBER = 11,
	DECEMBER = 12,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GuildScheduledEventException {
	pub event_id:             Snowflake,
	pub event_exception_id:   Snowflake,
	pub is_canceled:          bool,
	pub scheduled_start_time: Option<Timestamp>,
	pub scheduled_end_time:   Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GuildScheduledEventUser {
	pub guild_scheduled_event_id:           Snowflake,
	pub guild_scheduled_event_exception_id: Snowflake,
	pub response:                           GuildScheduledEventUserResponse,
	pub user_id:                            Snowflake,
	pub user:                               User,
	pub member:                             GuildMember,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventUserResponse {
	UNINTERESTED = 0,
	INTERESTED = 1,
}
