use dioxus::prelude::*;

use crate::start_websocket;
use crate::utils::local_storage::get_value;

#[component]
pub fn Home() -> Element {
	let navigator = use_navigator();

	start_websocket(&get_value("token").unwrap());

	rsx! {
		button {
			onclick: move |_| {
			    navigator.replace("/app");
			},
			id: "redirect",
			"Open in browser"
		}
	}
}
