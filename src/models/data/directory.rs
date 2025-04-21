#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use crate::models::types::Snowflake;

#[derive(Serialize, Deserialize, Default)]
pub struct DirectoryEntry {
	/// https://docs.discord.sex/resources/directory#directory-entry-type
	pub r#type:                u8,
	pub directory_channel_id:  Snowflake,
	pub entity_id:             Snowflake,
	pub created_at:            String,
	/// https://docs.discord.sex/resources/directory#directory-category
	pub primary_category_id:   u8,
	pub description:           Option<String>,
	pub author_id:             Snowflake,
	pub guild:                 Guild,
	pub guild_scheduled_event: GuildScheduledEvent,
}

pub enum EntryType {
	GUILD = 0,
	GUILD_SCHEDULED_EVENT = 1,
}

pub enum Category {
	UNCATEGORIZED = 0,
	SCHOOL_CLUB = 1,
	CLASS = 2,
	STUDY_SOCIAL = 3,
	// SUBJECT_MAJOR = 4,
	MISC = 5,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Guild {
	pub featurable_in_directory: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GuildScheduledEvent {
	pub guild:     Guild,
	pub user_rsvp: bool,
}
