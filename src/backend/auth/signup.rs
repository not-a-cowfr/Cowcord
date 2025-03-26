use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
	consent:       bool,
	date_of_birth: String,
	email:         String,
	fingerprint:   String,
	global_name:   String,
	invite:        Option<String>,
	password:      String,
	username:      String,
}

#[derive(Serialize, Deserialize)]
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

pub async fn register(info: ResgisterRequest) -> Result<RegisterResponse, Error> {
	let client = reqwest::Client::new();

	let response = client
		.post("https://discord.com/api/v9/auth/register")
		.json(&info)
		.send()
		.await?;

	let status = response.status();
	let response_text = response.text().await?;

	match status.as_u16() {
		| 200..299 => {
			console::log_1(&"200-299".into());
			let response: RequestResponse400 = serde_json::from_str(&response_text)?;
			return Ok(response);
		},
		| 400 => {
			let captcha_response: RequestResponse400 = serde_json::from_str(&response_text)?;

			// some fuckery goes on with like some documents requests that return like a key(?) thats then used in other requests, or idk maybe that key comes from somewhere else?
			// yeah reading the hcaptcha docs is miserable, gonna put this off for now since its not really even that useful

			let response = client
				.post(&format!(
					"https://api.hcaptcha.com/getcaptcha/{}",
					captcha_response.captcha_sitekey
				))
				.send()
				.await?;
			console::log_1(&format!("{:#?}", response.text().await?).into());
		},
		| _ => {
			// shit itself bc idk what to do yet for this
			console::log_1(&"something else".into());
		},
	}
}
