use std::error::Error;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::backend::save_value_to_storage;
use crate::components::modals::mfa_code::Modal;

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
	let mut show_modal = use_signal(|| false);
	let mut ticket = use_signal(|| None);

	rsx! {
		form {
			onsubmit: move |event: Event<FormData>| {
				event.prevent_default();
				console::log_1(&format!("test submit\n\n{:#?}", event).into());

				spawn(async move {
					let identifier = event.values().get("identifier")
						.and_then(|val| val.get(0).cloned())
						.unwrap_or_default();

					let password = event.values().get("password")
						.and_then(|val| val.get(0).cloned())
						.unwrap_or_default();

					let request = LoginRequest {
						login: identifier,
						password,
						undelete: None,
						login_source: None,
					};

					match login(request).await {
						| Ok(login_response) => {
							if login_response.mfa.unwrap_or(false) {
								if let Some(ticket_value) = login_response.ticket {
									ticket.set(Some(ticket_value));
									show_modal.set(true);
								} else {
									console::error_1(&"MFA required, but no ticket received!".into());
								}
							}
						},
						| Err(e) => console::error_1(&format!("Login failed: {}", e).into()),
					}
				});
			},
			input { name: "identifier", placeholder: "Email or Phone number" }
			input { name: "password", placeholder: "Password" }
			input { r#type: "submit", value: "Login" }
		}

		if show_modal() {
			Modal {
				on_submit: move |code: String| {
					show_modal.set(false);

					console::log_1(&"test mfa modal submit".into());

					if let Some(ticket_value) = ticket() {
						let mfa_request = MfaRequest {
							ticket: ticket_value,
							code,
							login_source: None,
						};

						spawn(async move {
							match mfa_login(mfa_request).await {
								| Ok(_login_response) => {}, // token is already saved in function so I dont _think_ this has any use
								| Err(e) => console::error_1(&format!("MFA Login failed: {}", e).into()),
							}
						});
					} else {
						console::error_1(&"No ticket found for MFA submission!".into());
					}
				}
			}
		}
	}
}
