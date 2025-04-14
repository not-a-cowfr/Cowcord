use serde::Deserialize;

use crate::models::types::Snowflake;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct StageInstance {
	pub id:                       Snowflake,
	pub guild_id:                 Snowflake,
	pub channel_id:               Snowflake,
	pub topic:                    String,
	/// https://docs.discord.sex/resources/guild#privacy-level
	pub privacy_level:            u8,
	pub invite_code:              Option<String>,
	#[deprecated]
	pub discoverable_disabled:    bool,
	pub guild_scheduled_event_id: Option<Snowflake>,
}
