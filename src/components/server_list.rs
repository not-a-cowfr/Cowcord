use dioxus::prelude::*;

#[component]
pub fn ServerList() -> Element {
	rsx! {
	   Outlet::<crate::Route> {}
	}
}
