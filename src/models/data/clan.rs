#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::enum_number;
use crate::models::types::Snowflake;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Clan {
	pub id:                         Snowflake,
	pub name:                       String,
	pub tag:                        String,
	pub icon_hash:                  Option<String>,
	pub member_count:               u8,
	pub description:                Option<String>,
	/// https://docs.discord.food/resources/clan#clan-play-style
	pub play_style:                 u8,
	pub search_terms:               Vec<String>,
	pub game_application_ids:       Vec<Snowflake>,
	/// https://docs.discord.food/resources/clan#clan-badge-type
	pub badge:                      u8,
	pub badge_hask:                 String,
	pub badge_color_primary:        String, // hex format
	pub badge_color_secondary:      String, // hex format
	/// https://docs.discord.food/resources/clan#clan-banner-style
	pub banner:                     u8,
	pub banner_hash:                String,
	pub brand_color_primary:        String, // hex format
	pub brand_color_secondary:      String, // hex format
	pub wildcard_descriptors:       Vec<String>,
	pub game_activity:              HashMap<Snowflake, GameActivity>,
	pub discovery_profile_features: Vec<String>,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum PlayStyle {
		NONE = 0,
		SOCIAL = 1,
		CASUAL = 2,
		COMPETITIVE = 3,
		CREATIVE = 4,
		VERY_COMPETITIVE = 5,
		_ => Unknown(u8),
	}
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum BadgeType {
		SWORD = 0,
		WATER_DROP = 1,
		SKULL = 2,
		TOADSTOOL = 3,
		MOON = 4,
		LIGHTNING = 5,
		HEART = 6,
		LEAF = 7,
		FIRE = 8,
		COMPASS = 9,
		CROSSHAIRS = 10,
		FLOWER = 11,
		FORCE = 12,
		GEM = 13,
		LAVA = 14,
		PSYCHIC = 15,
		SMOKE = 16,
		SNOW = 17,
		SOUND = 18,
		SUN = 19,
		WIND = 20,
		_ => Unknown(u8),
	}
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum BannerStyle {
		NIGHT_SKY = 0,
		CASTLE = 1,
		WORLD_MAP = 2,
		SEA_FOAM = 3,
		WARP_TUNNEL = 4,
		HOUSE = 5,
		HEIGHT_MAP = 6,
		MESH = 7,
		SPATTER = 8,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GameActivity {
	pub activity_level: u16,
	pub activity_score: u16,
}
