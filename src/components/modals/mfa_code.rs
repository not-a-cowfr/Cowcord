use dioxus::prelude::*;

#[component]
pub fn Modal(on_submit: EventHandler<String>) -> Element {
	let mut input_value = use_signal(|| "".to_string());

	rsx! {
		div {
			class: "modal-overlay",
			div {
				class: "modal-content",
				form {
					onsubmit: move |event: Event<FormData>| {
						event.prevent_default();

						let value = event.values().get("mfa_code")
							.and_then(|val| val.get(0).cloned())
							.unwrap_or_default();

						on_submit.call(value);
					},
					input {
						r#type: "text",
						name: "mfa_code",
						value: "{input_value}",
						oninput: move |event| input_value.set(event.value()),
						placeholder: "Enter MFA Code"
					},
					button { r#type: "submit", "Submit" }
				}
			}
		}
	}
}
