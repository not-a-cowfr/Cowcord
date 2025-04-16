mod login;
use std::error::Error;

pub use login::Login;
mod register;
pub use register::Register;
use serde::{Deserialize, Serialize};

use crate::utils::request::RequestClient;

#[derive(Serialize)]
pub struct SmsMfaRequest {
	pub token: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
struct SmsMfaResponse {
	phone: String, // phone number in response is redacted, eg. "+*******6080"
}

pub async fn send_sms_mfa(info: SmsMfaRequest) -> Result<String, Box<dyn Error>> {
	let client = RequestClient::new();

	let mfa_response: SmsMfaResponse = client.post("/mfa/sms/send", &info).await?;

	Ok(mfa_response.phone)
}
