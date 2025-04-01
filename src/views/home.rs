use dioxus::prelude::*;

use crate::utils::local_storage::get_value_from_storage;

#[component]
pub fn Home() -> Element {
	let navigator = use_navigator();

	let redirect = match get_value_from_storage("token") {
		| Some(_token) => "/@me",
		| None => "/login",
	};

	navigator.replace(redirect);

	rsx! {}
}
