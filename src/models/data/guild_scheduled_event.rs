#![allow(non_camel_case_types)]

use super::types::Timestamp;
use serde::Deserialize;

use super::guild::GuildMember;
use super::types::Snowflake;
use super::user::User;

#[derive(Deserialize, Default)]
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
	pub privacy_level:                    u8, // https://docs.discord.sex/resources/guild#privacy-level
	pub status:                           u8, // https://docs.discord.sex/resources/guild-scheduled-event#guild-scheduled-event-status
	pub entity_type:                      u8, // https://docs.discord.sex/resources/guild-scheduled-event#guild-scheduled-event-entity-type
	pub entity_id:                        Option<Snowflake>,
	pub entity_metadata:                  Option<EntityMetadata>,
	pub user_count:                       u32,
	pub image:                            Option<String>,
	pub recurrence_rule:                  Option<GuildScheduledEventRecurrenceRule>,
	pub guild_scheduled_event_exceptions: Vec<GuildScheduledEventException>,
}

pub enum GuildScheduledEventStatus {
	SCHEDULED = 1,
	ACTIVE = 2,
	COMPLETED = 3,
	CANCELED = 4,
}

pub enum GuildScheduledEventEntityType {
	STAGE_INSTANCE = 1,
	VOICE = 2,
	EXTERNAL = 3,
	PRIME_TIME = 4,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct EntityMetadata {
	pub location: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GuildScheduledEventRecurrenceRule {
	pub start:        Timestamp,
	pub end:          Option<Timestamp>,
	pub frequency:    u8, // https://docs.discord.sex/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule---frequency
	pub interval:     u16, // The spacing between the events, defined by frequency; for example, frequency of WEEKLY and an interval of 2 would be every other week
	pub by_weekday:   Option<Vec<u8>>,
	pub by_n_weekday: Option<Vec<NWeekday>>,
	pub by_month:     Option<Vec<u8>>,
	pub by_month_day: Option<Vec<u8>>,
	pub by_year_day:  Option<Vec<u16>>,
	pub count:        Option<u32>,
}

pub enum GuildScheduledEventRecurrenceRuleFrequency {
	YEARLY = 0,
	MONTHLY = 1,
	WEEKLY = 2,
	DAILY = 3,
}

pub enum GuildScheduledEventRecurrenceRuleWeekday {
	MONDAY = 0,
	TUESDAY = 1,
	WEDNESDAY = 2,
	THURSDAY = 3,
	FRIDAY = 4,
	SATURDAY = 5,
	SUNDAY = 6,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct NWeekday {
	pub n:   u8,
	pub day: u8,
}

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

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GuildScheduledEventException {
	pub event_id:             Snowflake,
	pub event_exception_id:   Snowflake,
	pub is_canceled:          bool,
	pub scheduled_start_time: Option<Timestamp>,
	pub scheduled_end_time:   Option<Timestamp>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GuildScheduledEventUser {
	pub guild_scheduled_event_id:           Snowflake,
	pub guild_scheduled_event_exception_id: Snowflake,
	pub response:                           u8, // https://docs.discord.sex/resources/guild-scheduled-event#guild-scheduled-event-user-response
	pub user_id:                            Snowflake,
	pub user:                               User,
	pub member:                             GuildMember,
}

pub enum GuildScheduledEventUserResponse {
	UNINTERESTED = 0,
	INTERESTED = 1,
}
