#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::channel::Channel;
use super::emoji::Emoji;
use super::invite::Invite;
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
	pub features:                   Vec<GuildFeatures>,
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
	pub primary_category:           DiscoveryCategory,
	pub keywords:                   Option<Vec<String>>,
	pub is_published:               bool,
	pub reasons_to_join:            Vec<DiscoveryReason>,
	pub social_links:               Option<Vec<String>>,
	pub about:                      Option<String>,
	pub category_ids:               Vec<Snowflake>,
	pub categories:                 Vec<DiscoveryCategory>,
	pub created_at:                 Timestamp,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DiscoveryRequirements {
	pub guild_id:                        Snowflake,
	pub safe_environment:                bool,
	pub healthy:                         bool,
	pub health_score_pending:            bool,
	pub size:                            bool,
	pub nsfw_properties:                 DiscoveryNsfwProperties,
	pub protected:                       bool,
	pub sufficient:                      bool,
	pub sufficient_without_grace_period: bool,
	pub valid_rules_channel:             bool,
	pub retention_healthy:               bool,
	pub engagement_healthy:              bool,
	pub age:                             bool,
	/// in days
	pub minimum_age:                     Option<u16>,
	pub health_score:                    DiscoveryHealthScore,
	pub minimum_size:                    Option<u32>,
	pub grace_period_end_date:           Timestamp,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DiscoveryNsfwProperties {
	pub channels:                    Vec<Snowflake>,
	pub channel_banned_keywords:     HashMap<Snowflake, Vec<String>>,
	pub name:                        String,
	pub name_banned_keywords:        Vec<String>,
	pub description:                 String,
	pub description_banned_keywords: Vec<String>,
}

/// Activity metrics are recalculated weekly, as an 8-week rolling average. If they are not yet eligible to be calculated, all fields will be null.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DiscoveryHealthScore {
	pub avg_nonnew_communicators: Option<String>,
	pub avg_nonnew_participators: Option<String>,
	pub num_intentful_joiners:    Option<String>,
	pub perc_ret_w1_intentful:    Option<f64>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DiscoveryMetadata {
	pub guild_id:                      Snowflake,
	pub primary_category_id:           u16,
	pub keywords:                      Option<Vec<String>>,
	pub emoji_discoverability_enabled: bool,
	pub partner_actioned_timestamp:    Option<Timestamp>,
	pub partner_application_timestamp: Option<Timestamp>,
	pub is_published:                  bool,
	pub reasons_to_join:               Vec<DiscoveryReason>,
	pub social_links:                  Option<Vec<String>>,
	pub about:                         Option<String>,
	pub category_ids:                  Vec<Snowflake>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DiscoveryReason {
	pub reason:     String,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DiscoveryCategory {
	pub id:         u16,
	pub name:       String,
	pub is_primary: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MonetizationStorePage {
	pub guild:             StorePageGuild,
	pub role_subscription: StorePageRoleSubscription,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StorePageGuild {
	pub id:                         Snowflake,
	pub name:                       String,
	pub icon_hash:                  Option<String>,
	pub approximate_member_count:   u32,
	pub approximate_presence_count: u32,
	pub locked_server:              bool,
	pub invite:                     Option<Invite>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StorePageRoleSubscription {
	/// pub settings:             RoleSubscriptionSettings,
	pub settings:             Value,
	/// pub group_listings:       Vec<RoleSubcriptionGroupListing>,
	pub group_listings:       Vec<Value>,
	// pub trials:               Vec<RoleSubcriptionTrial>,
	pub trials:               Vec<Value>,
	pub subscriber_count:     Option<u32>,
	pub benefit_channels:     Vec<Channel>,
	pub benefit_emojis:       Vec<Emoji>,
	pub purchase_page_invite: Option<Invite>,
}
