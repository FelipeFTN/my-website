#![allow(non_snake_case)]
#![allow(unused_imports)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info, error};

mod integrations;
mod components;

static CSS: Asset = asset!("/assets/styles/main.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    // Init environment variables
    // config::environment::init().expect("failed to init environment");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {},
        document::Stylesheet { href: CSS },
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Main {},
    }
}

#[component]
pub fn Main() -> Element {
    rsx! {
        main {
            // I hate using hard-coded values like this!
            // div { min_width: "330px", max_width: "400px" }
            components::sidebar::SideBar {}
            div { class: "main-content",
                components::projects::Projects {},
                components::games::Games {},
                components::blog::Blog {},
                components::books::Books {},
            }
            div {  class: "div-to-align-the-layout" }
        }
    }
}
