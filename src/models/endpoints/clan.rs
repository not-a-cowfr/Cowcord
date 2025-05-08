#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::models::data::clan::Clan;
use crate::models::data::guild::MemberVerificationForm;
use crate::models::data::user::User;
use crate::models::types::Snowflake;

/// Type: put
pub const SET_CLAN_IDENTITY_ENDPOINT: &str = "/users/@me/clan";

#[derive(Serialize)]
pub struct SetClanIdentityRequest {
	pub identity_enabled:  bool,
	pub identity_guild_id: Option<Snowflake>,
}

pub type SetClanIdentityResponse = User;

/// Type: get
pub fn GET_CLAN_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/discovery/{}/clan", guild_id)
}

pub type GetClainResponse = Clan;

/// Type: post
pub fn CREATE_CLAN_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/clan/{}", guild_id)
}

#[derive(Serialize)]
pub struct CreateClanRequest {
	pub tag:                   String,
	pub game_application_ids:  Vec<Snowflake>,
	pub search_terms:          Vec<String>,
	/// https://docs.discord.food/resources/clan#clan-play-style
	pub play_style:            u8,
	pub description:           String,
	pub wildcard_descriptors:  Vec<String>,
	/// https://docs.discord.food/resources/clan#clan-badge-type
	pub badge:                 u8,
	pub badge_color_primary:   String,
	pub badge_color_secondary: String,
	/// https://docs.discord.food/resources/clan#clan-banner-style
	pub banner:                u8,
	pub brand_color_primary:   String,
	pub brand_color_secondary: String,
	pub verification_form:     MemberVerificationForm,
}

/// Type: get
pub fn GET_CLAN_SETTINGS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/clan/{}/settings", guild_id)
}

#[derive(Deserialize)]
pub struct CreateClanResponse {
	pub tag:                   String,
	pub game_application_ids:  Vec<Snowflake>,
	pub search_terms:          Vec<String>,
	/// https://docs.discord.food/resources/clan#clan-play-style
	pub play_style:            u8,
	pub description:           String,
	pub wildcard_descriptors:  Vec<String>,
	/// https://docs.discord.food/resources/clan#clan-badge-type
	pub badge:                 u8,
	pub badge_color_primary:   String,
	pub badge_color_secondary: String,
	/// https://docs.discord.food/resources/clan#clan-banner-style
	pub banner:                u8,
	pub brand_color_primary:   String,
	pub brand_color_secondary: String,
	pub verification_form:     MemberVerificationForm,
}

/// Type: patch
pub fn MODIFY_CLAN_SETTINGS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/clan/{}/settings", guild_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyClanRequest {
	pub tag:                   String,
	pub game_application_ids:  Vec<Snowflake>,
	pub search_terms:          Vec<String>,
	/// https://docs.discord.food/resources/clan#clan-play-style
	pub play_style:            u8,
	pub description:           String,
	pub wildcard_descriptors:  Vec<String>,
	/// https://docs.discord.food/resources/clan#clan-badge-type
	pub badge:                 u8,
	pub badge_color_primary:   String,
	pub badge_color_secondary: String,
	/// https://docs.discord.food/resources/clan#clan-banner-style
	pub banner:                u8,
	pub brand_color_primary:   String,
	pub brand_color_secondary: String,
	/// only the form_fields and optionally description fields are accepted
	pub verification_form:     MemberVerificationForm,
}

/// Type: post
pub fn DISABLE_CLAN_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/clan/{}/disable", guild_id)
}
