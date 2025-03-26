use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
	rsx! {
		script { src: "https://js.hcaptcha.com/1/api.js" },
		body {
			form {
				method: "POST",
				action: "",
				input { r#type: "text", name: "email", placeholder: "Email" }
				input { r#type: "password", name: "password", placeholder: "Password" }
				div { class: "h-captcha", "data-sitekey": "your_site_key" }
				br {}
				input { r#type: "submit", value: "Submit" }
			}
		}
	}
}
