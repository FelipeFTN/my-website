#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
use once_cell::sync::Lazy;

mod components;
mod config;

// Define a global variable to store the configuration
static CONFIG: Lazy<config::config::AppConfig> = Lazy::new(|| config::config::AppConfig::new());

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
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
    // Access the configuration from the global variable
    // let config = CONFIG.clone();
    // or
    // CONFIG.get("key")

    rsx! {
        main {
            components::sidebar::SideBar {},
            components::projects::Projects {},
            div { class: "div-to-align-the-layout"},
        }
    }
}
