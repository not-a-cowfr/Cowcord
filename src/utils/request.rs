use std::error::Error;

use reqwest::{Client, Response, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct RequestClient {
	client:   Client,
	api_base: String,
}

const DISCORD: &str = "https://discord.com/";
const API_VERSION: &str = "9";

impl RequestClient {
	pub fn new() -> Self {
		RequestClient {
			client:   Client::new(),
			api_base: format!("/api/v{}", API_VERSION),
		}
	}

	async fn handle_response<T>(response: Response) -> Result<T, Box<dyn Error>>
	where
		T: DeserializeOwned,
	{
		let status = response.status();
		let response_text = response.text().await?;

		if status.is_success() {
			let result: T = serde_json::from_str(&response_text)?;
			Ok(result)
		} else {
			Err(format!(
				"Request failed with status: {}. Response: {}",
				status, response_text
			)
			.into())
		}
	}

	pub async fn post<T, R>(
		&self,
		endpoint: &str,
		body: &T,
	) -> Result<R, Box<dyn Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD, self.api_base, endpoint);

		let response = self
			.client
			.post(&url)
			.json(body)
			.header("Origin", DISCORD)
			.send()
			.await?;

		Self::handle_response(response).await
	}

	pub async fn get<R>(
		&self,
		endpoint: &str,
	) -> Result<R, Box<dyn Error>>
	where
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD, self.api_base, endpoint);

		let response = self
			.client
			.get(&url)
			.header("Origin", DISCORD)
			.send()
			.await?;

		Self::handle_response(response).await
	}

	pub async fn delete<R>(
		&self,
		endpoint: &str,
	) -> Result<R, Box<dyn Error>>
	where
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD, self.api_base, endpoint);

		let response = self
			.client
			.delete(&url)
			.header("Origin", DISCORD)
			.send()
			.await?;

		Self::handle_response(response).await
	}
}
