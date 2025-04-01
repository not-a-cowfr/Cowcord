use dioxus::prelude::*;

mod views;
use views::*;

pub mod components;
use components::*;

pub mod models;
pub mod utils;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/register")]
    Register {},

    #[layout(ServerList)]
        #[nest("/:server_id")]
        #[layout(Server)]
            #[route("/")]
            Roles { server_id: u64 },

            #[route("/:channel_id")]
            Channel { server_id: u64, channel_id: u64 },
        #[end_layout]
        #[end_nest]

        #[nest("/@me")]
        // #[layout()]
            // todo
        // #[end_layout]
        #[end_nest]
    #[end_layout]
} // literally no clue why theres an error here, from the examples ive seen this is how nesting and layouts are done unless it changed super recently but the documentation for this sucks
// error does go away if your remove the end_ stuff though so idk

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() { dioxus::launch(App); }

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }

		Router::<Route> {}
	}
}
