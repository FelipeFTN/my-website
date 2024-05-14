#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod components;

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
    rsx! {
        main {
            components::sidebar::SideBar {},
            components::projects::Projects {},
        }
    }
}
