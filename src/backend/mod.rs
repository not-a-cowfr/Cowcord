use wasm_bindgen::JsValue;
use web_sys::window;

pub mod auth;
pub mod websocket;

// this is supposed to be used for anything that needs token so that if anything happens to the token from local storage then the next request that needs it will get no token and then user gets redirected to /login
// pub fn get_token() -> String {
// 	if let Some(token) = get_value_from_storage("token") {
// 		return token;
// 	} else {
// 		// redirect to login page and idk cancel request or smth?
// 	}
// }

pub fn get_value_from_storage(key: &str) -> Option<String> {
	if let Some(window) = window() {
		if let Ok(local_storage) = js_sys::Reflect::get(&window, &JsValue::from_str("localStorage"))
		{
			if let Ok(token_value) = js_sys::Reflect::get(&local_storage, &JsValue::from_str(key)) {
				if !token_value.is_undefined() && !token_value.is_null() {
					if let Some(token_str) = token_value.as_string() {
						if !token_str.is_empty() {
							return Some(token_str);
						}
					}
				}
			}
		}
	}
	None
}

pub fn save_value_to_storage(
	key: &str,
	value: &str,
) {
	if let Some(window) = window() {
		if let Ok(local_storage) = js_sys::Reflect::get(&window, &JsValue::from_str("localStorage"))
		{
			let _ = js_sys::Reflect::set(
				&local_storage,
				&JsValue::from_str(key),
				&JsValue::from_str(value),
			);
		}
	}
}

pub fn remove_value_from_storage(key: &str) {
	if let Some(window) = window() {
		if let Ok(local_storage) = js_sys::Reflect::get(&window, &JsValue::from_str("localStorage"))
		{
			let remove_item = JsValue::from_str("removeItem");
			let token_key = js_sys::Array::new();
			token_key.push(&JsValue::from_str(key));
			let _ =
				js_sys::Function::from(js_sys::Reflect::get(&local_storage, &remove_item).unwrap())
					.apply(&local_storage, &token_key);
		}
	}
}
