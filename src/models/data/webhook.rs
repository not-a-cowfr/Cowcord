#![allow(non_camel_case_types)]

use serde::Deserialize;

use super::integration::IntegrationGuild;
use crate::models::types::Snowflake;
use super::user::User;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Webhook {
	pub id:             Snowflake,
	/// https://docs.discord.sex/resources/webhook#webhook-types
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

pub enum WebcookType {
	INCOMING = 1,
	CHANNEL_FOLLOWER = 2,
	APPLICATION = 3,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct WebhookChannel {
	pub id:   Snowflake,
	pub name: String,
}
