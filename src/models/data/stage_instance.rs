use serde::{Deserialize, Serialize};

use super::guild::PrivacyLevel;
use crate::models::types::Snowflake;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StageInstance {
	pub id:                       Snowflake,
	pub guild_id:                 Snowflake,
	pub channel_id:               Snowflake,
	pub topic:                    String,
	pub privacy_level:            PrivacyLevel,
	pub invite_code:              Option<String>,
	#[deprecated]
	pub discoverable_disabled:    bool,
	pub guild_scheduled_event_id: Option<Snowflake>,
}
