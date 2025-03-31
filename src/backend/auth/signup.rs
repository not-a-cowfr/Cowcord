use std::error::Error;

use serde::{Deserialize, Serialize};
use web_sys::console;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
	pub consent:       bool,
	pub date_of_birth: String,
	pub email:         String,
	pub fingerprint:   String,
	pub global_name:   String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invite:        Option<String>,
	pub password:      String,
	pub username:      String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestResponse400 {
	captcha_key: Vec<String>,
	captcha_sitekey: String,
	captcha_service: String,
	should_serve_invisible: bool,
	captcha_rqdata: String,
	captcha_rqtoken: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
	token: String,
}

pub async fn register(info: RegisterRequest) -> Result<(), Box<dyn Error>> {
	let client = reqwest::Client::new();

	let response = client
		.post("https://discord.com/api/v9/auth/register")
		.json(&info)
		.send()
		.await
		.map_err(|e| Box::new(e) as Box<dyn Error>)?;

	let status = response.status();
	let response_text = response
		.text()
		.await
		.map_err(|e| Box::new(e) as Box<dyn Error>)?;

	console::log_1(&format!("{:#?}", response_text).into());

	match status.as_u16() {
		| 200..=299 => {
			console::log_1(&"200-299".into());
			let response: RequestResponse400 =
				serde_json::from_str(&response_text).map_err(|e| Box::new(e) as Box<dyn Error>)?;
			console::log_1(&format!("{:#?}", response).into());
		},
		| 400 => {
			let captcha_response: RequestResponse400 =
				serde_json::from_str(&response_text).map_err(|e| Box::new(e) as Box<dyn Error>)?;

			// some fuckery goes on with like some documents requests that return like a key(?) thats then used in other requests, or idk maybe that key comes from somewhere else?
			// yeah reading the hcaptcha docs is miserable, gonna put this off for now since its not really even that useful

			let response = client
				.post(&format!(
					"https://api.hcaptcha.com/getcaptcha/{}",
					captcha_response.captcha_sitekey
				))
				.send()
				.await
				.map_err(|e| Box::new(e) as Box<dyn Error>)?;
			console::log_1(
				&format!(
					"{:#?}",
					response
						.text()
						.await
						.map_err(|e| Box::new(e) as Box<dyn Error>)?
				)
				.into(),
			);
		},
		| _ => {
			// shit itself bc idk what to do yet for this
			console::log_1(&"something else".into());
		},
	}
	Ok(())
}
