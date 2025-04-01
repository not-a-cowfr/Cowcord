use serde::Deserialize;

// json response when mfa is required for an action
#[derive(Deserialize)]
pub struct MfaRequired {
	pub message: String,
	pub code:    u64,
	pub mfa:     MfaObject,
}

#[derive(Deserialize)]
pub struct MfaObject {
	pub ticket:  String,
	pub methods: Vec<MfaMethods>,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
pub enum MfaMethods {
	totp,
	sms,
	backup,
	webauthn,
	password,
}

#[derive(Deserialize)]
pub struct MfaMethod {
	pub r#type:                String,
	pub challenge:             Option<String>,
	pub backupd_codes_allowed: Option<String>,
}
