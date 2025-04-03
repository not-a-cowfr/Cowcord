use std::error::Error;

use dioxus::prelude::*;
use web_sys::console;

use crate::models::data::message::{Message, MessageHistoryRequest, MessageHistoryResponse};

async fn get_channel_messages(
	channel_id: u64,
	message_id: Option<String>,
) -> Result<Vec<Message>, Box<dyn Error>> {
	let mut request = MessageHistoryRequest {
		limit:  50,
		after:  None,
		around: None,
		before: None,
	};

	if message_id.is_some() {
		request.around = message_id;
	}

	let client = reqwest::Client::new();

	let response = client
		.get(format!(
			"https://discord.com/api/v9/channels/{}/messages",
			channel_id,
		))
		.json(&request)
		.send()
		.await?;

	let status = response.status();
	let response_text = response.text().await?;

	if status.is_success() {
		let message_history_response: MessageHistoryResponse =
			serde_json::from_str(&response_text)?;

		return Ok(message_history_response.messages);
	}

	// todo catch the permission errors for no VIEW_CHANNEL perms and no READ_MESSAGE_HISTORY perms
	Err(match status.as_u16() {
		| _ => format!("Unhandled status: {}", status).into(),
	})
}

#[component]
pub fn Chat(channel_id: u64) -> Element {
	spawn(async move {
		let messages = match get_channel_messages(channel_id, None).await {
			| Ok(messages) => Some(messages),
			| Err(e) => {
				console::log_1(&format!("{}", e).into());
				None
			},
		};
		console::log_1(&format!("{:#?}", messages).into());
	});

	rsx! {}
}
