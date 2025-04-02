use serde::{Deserialize, Serialize};

use crate::models::types::Snowflake;

// https://docs.discord.sex/authentication#login-source
#[derive(Serialize)]
#[allow(non_camel_case_types)]
pub enum LoginSource {
	gift,
	guild_template,
	guild_invite,
	dm_invite,
	friend_invite,
	role_subscription,
	role_subscription_setting,
}

#[derive(Serialize)]
pub struct LoginRequest {
	pub login:        String,
	pub password:     String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub undelete:     Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_source: Option<LoginSource>,
}

#[derive(Serialize)]
pub struct MfaRequest {
	pub ticket:       String,
	pub code:         String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_source: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct LoginResponse {
	pub user_id:          Snowflake,
	pub token:            Option<String>,
	pub user_settings:    Option<LoginSettings>,
	pub required_actions: Option<Vec<RequiredActions>>,
	pub ticket:           Option<String>,
	pub mfa:              Option<bool>,
	pub totp:             Option<bool>,
	pub sms:              Option<bool>,
	pub backup:           Option<bool>,
	pub webauthn:         Option<String>,
}

#[derive(Deserialize)]
pub struct LoginSettings {
	pub locale: String, // https://docs.discord.sex/reference#locales
	pub theme:  String, // "dark" or "light"
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum RequiredActions {
	update_password,
}
