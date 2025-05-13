#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
pub struct Session {
	pub session_id:  String,
	pub client_info: ClientInfo,
	pub status:      String,
	pub activities:  Vec<Activity>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub active:      Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ClientInfo {
	pub client:  String,
	pub os:      String,
	pub version: i64,
}

pub enum ClientType {
	desktop,
	web,
	mobile,
	unknown,
}

pub enum OperatingSystemType {
	windows,
	osx,
	linux,
	android,
	ios,
	playstation,
	unknown,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ClientStatus {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub desktop:  Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile:   Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub web:      Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub embedded: Option<String>,
}

pub enum StatusType {
	online,
	dnd,
	idle,
	invisible,
	offline,
	unknown,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Activity {
	pub id:                  String,
	pub name:                String,
	pub r#type:              ActivityType,
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
	pub flags:               ActivityFlags,
	pub buttons:             Vec<String>,
	pub emoji:               Option<ActivityEmoji>,
	pub party:               ActivityParty,
	pub assets:              ActivityAssets,
	pub secrets:             ActivitySecrets,
	pub metadata:            ActivityMetadata,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActivityType {
	PLAYING = 0,
	STREAMING = 1,
	LISTENING = 2,
	WATCHING = 3,
	CUSTOM = 4,
	COMPETING = 5,
	HANG = 6,
}

pub enum ActivityPlatformType {
	desktop,
	xbox,
	samsung,
	ios,
	android,
	embedded,
	ps4,
	ps5,
}

bitflags! {
	pub struct ActivityFlags: u64 {
		const INSTANCE = 1 << 0;
		const JOIN = 1 << 1;
		#[deprecated]
		const SPECTATE = 1 << 2;
		const JOIN_REQUEST = 1 << 3;
		const SYNC = 1 << 4;
		const PLAY = 1 << 5;
		const PARTY_PRIVACY_FRIENDS = 1 << 6;
		const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;
		const EMBEDDED = 1 << 8;
	}
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActivityActionType {
	JOIN = 1,
	SPECTATE = 2,
	LISTEN = 3,
	WATCH = 4,
	JOIN_REQUEST = 5,
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
	/// https://docs.discord.food/resources/presence#activity-asset-image
	pub small_image: String,
	pub small_text:  String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ActivitySecrets {
	pub join: String,
}

/// Activity metadata can consist of arbitrary data, and is not sanitized by the API. Treat data within this object carefully.
///
/// The below structure is only a convention that is used by official clients. It is not enforced by the API.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ActivityMetadata {
	pub button_urls: Vec<String>,
	pub artist_ids:  Vec<String>,
	pub album_id:    String,
	pub context_uri: String,
	pub r#type:      String,
}
