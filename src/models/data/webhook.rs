#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use super::integration::IntegrationGuild;
use super::user::User;
use crate::enum_number;
use crate::models::types::Snowflake;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Webhook {
	pub id:             Snowflake,
	/// https://docs.discord.food/resources/webhook#webhook-types
	pub r#type:         u8,
	pub guild_id:       Option<Snowflake>,
	pub channel_id:     Option<Snowflake>,
	pub user:           Option<User>,
	pub name:           Option<String>,
	pub avatar:         Option<String>,
	pub token:          String,
	pub application_id: Option<Snowflake>,
	pub source_guild:   IntegrationGuild,
	pub source_channel: WebhookChannel,
	pub url:            String,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum WebcookType {
		INCOMING = 1,
		CHANNEL_FOLLOWER = 2,
		APPLICATION = 3,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct WebhookChannel {
	pub id:   Snowflake,
	pub name: String,
}
