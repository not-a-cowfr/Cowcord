use gloo_timers::callback::Interval;
use serde_json::{Value, json};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{BinaryType, MessageEvent, WebSocket, console};

#[allow(non_snake_case)]
pub struct Ready {
	session_id:         String,
	connected_accounts: Vec<String>, // probably wrong
	sessions:           Vec<Session>,
	tutorial:           Option<bool>, // idk about this
}

pub struct Session {
	status:      String,
	session_id:  String,
	client_info: ClientInfo,
	activities:  Vec<Activity>, // idk what this one is
}

pub struct ClientInfo {
	version: u64,
	os:      String,
	client:  String,
}

pub struct Activity {}

// TODO: make it so that if the websocket fails bc of bad token but the user is logged in, delete the stored token and make them log back in
pub fn start_websocket(token: &str) {
	console::log_1(&"Starting WebSocket connection...".into());
	let gateway_url = "wss://gateway.discord.gg/?encoding=json&v=9";

	let ws = WebSocket::new(gateway_url).expect("Failed to create WebSocket");
	ws.set_binary_type(BinaryType::Arraybuffer);

	let sequence = Rc::new(RefCell::new(Option::<i64>::None));
	let heartbeat_interval = Rc::new(RefCell::new(None::<Interval>));

	let ws_clone = ws.clone();
	let sequence_clone = sequence.clone();
	let heartbeat_interval_clone = heartbeat_interval.clone();
	let token = token.to_string();

	let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
		if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
			let text_str = text.as_string().unwrap();
			let data: Value = serde_json::from_str(&text_str).expect("Failed to parse JSON");
			console::log_1(&format!("{}", &text_str).into());

			if let Some(op_code) = data["op"].as_u64() {
				match op_code {
					| 10 => {
						let interval_ms =
							data["d"]["heartbeat_interval"].as_f64().unwrap_or(41250.0);

						let ws_heartbeat = ws_clone.clone();
						let seq_for_heartbeat = sequence_clone.clone();

						if let Some(interval) = heartbeat_interval_clone.borrow_mut().take() {
							interval.cancel();
						}

						let interval = Interval::new(interval_ms as u32, move || {
							let seq_value = *seq_for_heartbeat.borrow();
							let heartbeat = json!({
								"op": 1,
								"d": seq_value
							});
							let _ = ws_heartbeat.send_with_str(&heartbeat.to_string());
							console::log_1(&"Heartbeat sent".into());
						});

						*heartbeat_interval_clone.borrow_mut() = Some(interval);

						let identify = json!({
							"op": 2,
							"d": {
								"token": token,
								"intents": 3276799,
								"properties": {
									// TODO: get real properties
									"os": "Windows",
									"browser": "Chrome",
									"device": ""
								}
							}
						});
						ws_clone
							.send_with_str(&identify.to_string())
							.expect("Failed to send IDENTIFY");
					},
					| 0 => {
						// ready response
						console::log_1(&format!("{:#?}", data).into());
					},
					| _ => {
						console::log_1(&format!("Unhandled response: {:#?}", data).into());
					},
				}
			}

			if let Some(seq) = data["s"].as_i64() {
				*sequence_clone.borrow_mut() = Some(seq);
			}
		}
	}) as Box<dyn FnMut(MessageEvent)>);

	let onerror_callback = Closure::wrap(Box::new(move |e: web_sys::ErrorEvent| {
		console::error_1(&format!("Websocket error: {}", e.message()).into());
	}) as Box<dyn FnMut(web_sys::ErrorEvent)>);

	let onopen_callback = Closure::wrap(Box::new(move |_| {
		console::log_1(&"Websocket connection started".into());
	}) as Box<dyn FnMut(JsValue)>);

	let onclose_callback = Closure::wrap(Box::new(move |e: web_sys::CloseEvent| {
		console::log_1(&format!("Websocket closed: {} ({})", e.reason(), e.code()).into());
	}) as Box<dyn FnMut(web_sys::CloseEvent)>);

	ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
	ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
	ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
	ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));

	onmessage_callback.forget();
	onerror_callback.forget();
	onopen_callback.forget();
	onclose_callback.forget();
}
