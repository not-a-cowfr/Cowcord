use serde::Deserialize;

use super::user::settings::{NotificationSettings, UserSettings};
use super::user::user::User;

#[derive(Deserialize)]
pub struct Gateway {
	pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GatewayRecieveEvent {
	pub op: u8,
	pub d:  serde_json::Value,
	pub s:  Option<i64>,
	pub t:  Option<String>,
}

#[derive(Deserialize)]
pub struct Ready {
	_trace:                Vec<String>,
	v:                     u8,
	user:                  User,
	#[deprecated]
	user_settings:         UserSettings,
	user_settings_proto:   String, // base 64 encoded, todo: parsing (i already tried it please kill me that was awful)
	notification_settings: NotificationSettings,
	// todoL finish https://docs.discord.sex/topics/gateway-events#ready-structure
}
