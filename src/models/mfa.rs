use serde::Deserialize;

// json response when mfa is required for an action
#[derive(Deserialize)]
pub struct MfaRequired {
	message: String,
	code:    u64,
	mfa:     MfaObject,
}

#[derive(Deserialize)]
pub struct MfaObject {
	ticket:  String,
	methods: Vec<MfaMethods>,
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
	r#type: String,
	challenge: Option<String>,
	backupd_codes_allowed: Option<String>,
}
