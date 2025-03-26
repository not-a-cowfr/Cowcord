use serde::{Deserialize, Serialize};
use std::error::Error;
use web_sys::console;

use crate::backend::save_value_to_storage;

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    login: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub mfa: Option<bool>,
    pub ticket: Option<String>,
}

// TODO: 2fa
pub async fn login(email: String, password: String) -> Result<LoginResponse, Box<dyn Error>> {
    let login_request = LoginRequest {
        login: email.clone(),
        password: password.clone(),
    };

    let client = reqwest::Client::new();

    let response = client
        .post("https://discord.com/api/v9/auth/login")
        .json(&login_request)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if status.is_success() {
        let login_response: LoginResponse = serde_json::from_str(&response_text)?;
        save_value_to_storage("token", &login_response.token);
        return Ok(login_response);
    }

    if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
        if let Some(message) = error_json.get("message").and_then(|m| m.as_str()) {
            if let Some(errors) = error_json.get("errors") {
                if let Some(login_errors) = errors.get("login") {
                    if let Some(error_list) = login_errors.get("_errors") {
                        if let Some(first_error) = error_list.get(0) {
                            if let Some(error_message) =
                                first_error.get("message").and_then(|m| m.as_str())
                            {
                                let code = first_error
                                    .get("code")
                                    .and_then(|c| c.as_str())
                                    .unwrap_or("");

                                return match code {
                                    "ACCOUNT_COMPROMISED_RESET_PASSWORD" => {
                                        Err(format!("Account security notice: {}", error_message)
                                            .into())
                                    }
                                    _ => Err(format!("Login error: {}", error_message).into()),
                                };
                            }
                        }
                    }
                }

                return Err(format!("Login failed: {}", message).into());
            }

            return Err(format!("Login failed: {}", message).into());
        }
    }

    Err(match status.as_u16() {
        401 => "Invalid email or password. Please try again.".into(),
        429 => "Too many login attempts. Please try again later.".into(),
        _ => format!("Login failed with status {}", status).into(),
    })
}
