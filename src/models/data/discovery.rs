#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::Deserialize;

use super::emoji::Emoji;
use super::sticker::Sticker;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DiscoverableGuild {
	pub id:                         Snowflake,
	pub name:                       String,
	pub icon:                       Option<String>,
	pub description:                Option<String>,
	pub banner:                     Option<String>,
	pub splash:                     Option<String>,
	pub discovery_splash:           Option<String>,
	/// https://docs.discord.sex/resources/guild#guild-features
	pub features:                   Vec<String>,
	pub vanity_url_code:            Option<String>,
	pub preferred_locale:           String,
	pub premium_subscription_count: u32,
	pub approximate_member_count:   u32,
	pub approximate_presence_count: u32,
	pub emojis:                     Vec<Emoji>,
	pub emoji_count:                u16,
	pub stickers:                   Vec<Sticker>,
	pub sticker_count:              u16,
	pub auto_removed:               bool,
	pub primary_category_id:        u16,
	pub primary_category:           Category,
	pub keywords:                   Option<Vec<String>>,
	pub is_published:               bool,
	pub reasons_to_join:            Vec<Reason>,
	pub social_links:               Option<Vec<String>>,
	pub about:                      Option<String>,
	pub category_ids:               Vec<Snowflake>,
	pub categories:                 Vec<Category>,
	pub created_at:                 Timestamp,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Requirements {
	pub guild_id:                        Snowflake,
	pub safe_environment:                bool,
	pub healthy:                         bool,
	pub health_score_pending:            bool,
	pub size:                            bool, // "size: bool"🥺 (it means whether or not guild meets member reqs)
	pub nsfw_properties:                 NsfwProperties,
	pub protected:                       bool,
	pub sufficient:                      bool,
	pub sufficient_without_grace_period: bool,
	pub valid_rules_channel:             bool,
	pub retention_healthy:               bool,
	pub engagement_healthy:              bool,
	pub age:                             bool,
	pub minimum_age:                     Option<u16>, // in days
	pub health_score:                    HealthScore,
	pub minimum_size:                    Option<u32>,
	pub grace_period_end_date:           Timestamp,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NsfwProperties {
	pub channels:                    Vec<Snowflake>,
	pub channel_banned_keywords:     HashMap<Snowflake, Vec<String>>,
	pub name:                        String,
	pub name_banned_keywords:        Vec<String>,
	pub description:                 String,
	pub description_banned_keywords: Vec<String>,
}

// Activity metrics are recalculated weekly, as an 8-week rolling average. If they are not yet eligible to be calculated, all fields will be null.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HealthScore {
	pub avg_nonnew_communicators: Option<String>,
	pub avg_nonnew_participators: Option<String>,
	pub num_intentful_joiners:    Option<String>,
	pub perc_ret_w1_intentful:    Option<f64>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Metadata {
	pub guild_id:                      Snowflake,
	pub primary_category_id:           u16,
	pub keywords:                      Option<Vec<String>>,
	pub emoji_discoverability_enabled: bool,
	pub partner_actioned_timestamp:    Option<Timestamp>,
	pub partner_application_timestamp: Option<Timestamp>,
	pub is_published:                  bool,
	pub reasons_to_join:               Vec<Reason>,
	pub social_links:                  Option<Vec<String>>,
	pub about:                         Option<String>,
	pub category_ids:                  Vec<Snowflake>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Reason {
	pub reason:     String,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Category {
	pub id:         u16,
	pub name:       String,
	pub is_primary: bool,
}
