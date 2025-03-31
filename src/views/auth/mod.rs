mod login;
use std::error::Error;

pub use login::Login;
mod register;
pub use register::Register;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SmsMfaRequest {
	token: String,
}

#[derive(Deserialize)]
struct SmsMfaResponse {
	phone: String, // phone number in response is redacted, eg. "+*******6080"
}

pub async fn send_sms_mfa(info: SmsMfaRequest) -> Result<String, Box<dyn Error>> {
	let client = reqwest::Client::new();

	let response = client
		.post("https://discord.com/mfa/sms/send")
		.json(&info)
		.send()
		.await?;

	let status = response.status();
	let response_text = response.text().await?;

	if status.is_success() {
		let mfa_response: SmsMfaResponse = serde_json::from_str(&response_text)?;
		return Ok(mfa_response.phone);
	}

	Err(match status.as_u16() {
		| _ => format!("Unhandled status: {}", status).into(),
	})
}
