#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::application::{ApplicationRoleConnectionMetadata, ApplicationSKU};
use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Integration {
	pub id:                        Snowflake,
	pub name:                      String,
	pub r#type:                    IntegrationType,
	pub enabled:                   bool,
	pub account:                   IntegrationAccount,
	pub syncing:                   bool,
	pub role_id:                   Snowflake,
	pub enable_emoticons:          bool,
	pub expire_behavior:           IntegrationExpireBehavior,
	/// in days
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

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IntegrationExpireBehavior {
	REMOVE_ROLE = 0,
	KICK = 1,
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
	pub r#type:                            Option<ApplicationType>,
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
