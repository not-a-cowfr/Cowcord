use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	let navigator = use_navigator();

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
