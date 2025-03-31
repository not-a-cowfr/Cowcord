use std::error::Error;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::save_value_to_storage;

// https://docs.discord.sex/authentication#login-source
#[derive(Serialize)]
enum LoginSource {
	// idfk what causes these warnings or how to fix/ignore them
	gift,
	guild_template,
	guild_invite,
	dm_invite,
	friend_invite,
	role_subscription,
	role_subscription_setting,
}

#[derive(Serialize)]
struct LoginRequest {
	login:        String,
	password:     String,
	#[serde(skip_serializing_if = "Option::is_none")]
	undelete:     Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	login_source: Option<LoginSource>,
}

#[derive(Serialize)]
struct MfaRequest {
	ticket:       String,
	code:         String,
	#[serde(skip_serializing_if = "Option::is_none")]
	login_source: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct LoginResponse {
	user_id: String,
	token: Option<String>,
	user_settings: Option<LoginSettings>,
	required_actions: Option<Vec<RequiredActions>>,
	ticket: Option<String>,
	mfa: Option<bool>,
	totp: Option<bool>,
	sms: Option<bool>,
	backup: Option<bool>,
	webauthn: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginSettings {
	locale: String,
	theme:  String,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
enum RequiredActions {
	update_password,
}

async fn login(info: LoginRequest) -> Result<LoginResponse, Box<dyn Error>> {
	let client = reqwest::Client::new();

	let response = client
		.post("https://discord.com/api/v9/auth/login")
		.json(&info)
		.send()
		.await?;

	let status = response.status();
	let response_text = response.text().await?;

	if status.is_success() {
		let login_response: LoginResponse = serde_json::from_str(&response_text)?;
		if let Some(token) = &login_response.token {
			save_value_to_storage("token", token);
		}
		return Ok(login_response);
	}

	Err(match status.as_u16() {
		| 401 => "Invalid email or password. Please try again.".into(),
		| 429 => "Too many login attempts. Please try again later.".into(),
		| _ => format!("Unhandled status: {}", status).into(),
	})
}

async fn mfa_login(info: MfaRequest) -> Result<LoginResponse, Box<dyn Error>> {
	let client = reqwest::Client::new();

	let response = client
		.post(format!(
			"https://discord.com/api/v9/auth/mfa/{}",
			info.code.to_lowercase()
		))
		.json(&info)
		.send()
		.await?;

	let status = response.status();
	let response_text = response.text().await?;

	if status.is_success() {
		let login_response: LoginResponse = serde_json::from_str(&response_text)?;
		if let Some(token) = &login_response.token {
			save_value_to_storage("token", token);
		}
		return Ok(login_response);
	}

	Err(match status.as_u16() {
		| _ => format!("Unhandled status: {}", status).into(),
	})
}

#[component]
pub fn Login() -> Element {
	rsx! {
		form {
			onsubmit: async move |event: Event<FormData>| {
				event.prevent_default();

				console::log_1(&format!("{event:#?}").into());

				// TODO: phone number must be formatted with E.164 format
				let identifier: String = event.values().get("identifier")
					.and_then(|val| val.get(0).cloned())
					.unwrap_or_else(|| "".to_string());

				let password: String = event.values().get("password")
					.and_then(|val| val.get(0).cloned())
					.unwrap_or_else(|| "".to_string());

				let request = LoginRequest {
					login: identifier,
					password: password,
					undelete: None,
					login_source: None,
				};

				match login(request).await {
					| Ok(login_response) => {
						if login_response.mfa.unwrap_or(false) {
							if let Some(ticket) =  login_response.ticket.unwrap() {

								// using 2 matches when 1 could be used is kinda stupid, but I dont feel like changing it with the excuse of that it might be useful
								let mfa_type = match (
									login_response.totp,
									login_response.sms,
									login_response.backup,
								) {
									| (Some(true), _, _) => "TOTP",
									| (_, Some(true), _) => "SMS",
									| (_, _, Some(true)) => "backup",
									| _ => {
										console::error_1(&"WebAuthn mfa is currently unsupported!".into());
										panic!()
									},
								};

								match mfa_type {
									| "SMS" => send_sms_mfa(ticket),
								}

								// need to first create the popup modal for mfa thingys then get the code from that modal, also prob move the rest of this logic to that modal?
								// ui and dioxus components are not my strong suit (yet) so im gonna delay doing that

								let mfa_request = MfaRequest {
									ticket: ticket,
									code: /* needs to be gotten from the mfa modal */,
									login_source: None,
								};

								match mfa_login(mfa_request).await {
									| Ok(_login_response) => {}, // token is already saved in function so I dont _think_ this has any use
									| Err(e) => console::error_1(&format!("Login failed: {}", e).into()),
								}
							}
						}
					}
					| Err(e) => console::error_1(&format!("Login failed: {}", e).into()),
				}
			},
			input { name: "identifier", placeholder: "Email or Phone number", class: "email" }
			input { name: "password", placeholder: "Password", class: "password" }
			input { r#type: "submit" }
		}
	}
}
