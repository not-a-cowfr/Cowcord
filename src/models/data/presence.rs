use serde::{Deserialize, Serialize};

use super::user::User;
use crate::models::types::Snowflake;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Presence {
	pub user:          User,
	pub guild_id:      Snowflake,
	pub status:        String,
	pub activities:    Vec<Activity>,
	pub client_status: ClientStatus,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Activity {
	pub id:                  String,
	pub name:                String,
	/// https://docs.discord.food/resources/presence#activity-type
	pub r#type:              u8,
	pub url:                 Option<String>,
	pub created_at:          u32,
	pub session_id:          Option<String>,
	pub platform:            String,
	pub supported_platforms: Vec<String>,
	pub timestamps:          ActivityTimestamps,
	pub application_id:      Snowflake,
	pub details:             Option<String>,
	pub state:               Option<String>,
	pub sync_id:             String,
	/// https://docs.discord.food/resources/presence#activity-flags
	pub flags:               u64,
	pub buttons:             Vec<String>,
	pub emoji:               Option<ActivityEmoji>,
	pub party:               ActivityParty,
	pub assets:              ActivityAssets,
	pub secrets:             ActivitySecrets,
	pub metadata:            ActivityMetadata,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ActivityTimestamps {
	pub start: String,
	pub end:   String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ActivityEmoji {
	pub name:     String,
	pub id:       Snowflake,
	pub animated: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ActivityParty {
	pub id:   String,
	pub size: Vec<(u32, u32)>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ActivityAssets {
	/// https://docs.discord.food/resources/presence#activity-asset-image
	pub large_image: String,
	pub large_text:  String,
	pub small_image: String,
	pub small_text:  String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ActivitySecrets {
	pub join: String,
}

// Activity metadata can consist of arbitrary data, and is not sanitized by the API. Treat data within this object carefully.
//
// The below structure is only a convention that is used by official clients. It is not enforced by the API.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ActivityMetadata {
	pub button_urls: Vec<String>,
	pub artist_ids:  Vec<String>,
	pub album_id:    String,
	pub context_uri: String,
	pub r#type:      String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ClientStatus {
	pub desktop:  String,
	pub mobile:   String,
	pub web:      String,
	pub embedded: String,
}
