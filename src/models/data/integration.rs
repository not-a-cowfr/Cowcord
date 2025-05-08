#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use super::application::{ApplicationRoleConnectionMetadata, ApplicationSKU};
use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Integration {
	pub id:                        Snowflake,
	pub name:                      String,
	/// https://docs.discord.food/resources/integration#integration-type
	pub r#type:                    String,
	pub enabled:                   bool,
	pub account:                   IntegrationAccount,
	pub syncing:                   bool,
	pub role_id:                   Snowflake,
	pub enable_emoticons:          bool,
	/// https://docs.discord.food/resources/integration#integration-expire-behavior
	pub expire_behavior:           u8,
	/// https://docs.discord.food/topics/oauth2#oauth2-scopes
	pub expire_grace_period:       u8,
	pub synced_at:                 Timestamp,
	pub subscriber_count:          u32,
	pub revoked:                   bool,
	pub application:               IntegrationApplication,
	pub scopes:                    Vec<String>,
	pub role_connections_metadata: Vec<ApplicationRoleConnectionMetadata>,
	pub user:                      User,
}

pub enum IntegrationType {
	twitch,
	youtube,
	discord,
	guild_subscription,
}

pub enum IntegrationExpireBehavior {
	REMOVE_ROLE,
	KICK,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IntegrationAccount {
	pub id:   String,
	pub name: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IntegrationApplication {
	pub id:                                Snowflake,
	pub name:                              String,
	pub description:                       String,
	pub icon:                              Option<String>,
	pub cover_image:                       String,
	pub splash:                            String,
	/// https://docs.discord.food/resources/integration#integration-type
	pub r#type:                            Option<u8>,
	pub primary_sku_id:                    Snowflake,
	pub bot:                               User,
	pub deeplink_uri:                      Option<String>,
	pub third_party_skus:                  Vec<ApplicationSKU>,
	pub role_connections_verification_url: Option<String>,
	pub is_verified:                       bool,
	pub is_discoverable:                   bool,
	pub is_monetized:                      bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IntegrationGuild {
	pub id:   Snowflake,
	pub name: String,
	pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Gif {
	pub id:      String,
	#[deprecated]
	pub title:   String,
	pub url:     String,
	pub src:     String,
	pub gif_src: String,
	pub preview: String,
	pub width:   u16,
	pub height:  u16,
}
