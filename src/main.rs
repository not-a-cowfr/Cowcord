use dioxus::prelude::*;

mod views;
use views::*;

pub mod components;
use components::*;

pub mod models;
pub mod utils;

// https://github.com/DioxusLabs/dioxus/issues/3211
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {
    #[layout(ServerList)]
        #[layout(Server)]
            #[route("/:server_id")]
            Roles { server_id: u64 },

            #[route("/:server_id/:channel_id")]
            Channel { server_id: u64, channel_id: u64 },
        #[end_layout]

        #[nest("/@me")]
        // #[layout()]
            // todo
        // #[end_layout]
        #[end_nest]
    #[end_layout]

    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/register")]
    Register {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() { dioxus::launch(App); }

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }

		Router::<Route> {}
	}
}
