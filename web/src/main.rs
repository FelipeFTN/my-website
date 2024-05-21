#![allow(non_snake_case)]
#![allow(unused_imports)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info, error};

mod integrations;
mod components;
// Fuck env variables for now - I'll do it later.
// mod config;

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
    let click = move |_| {
        spawn(async move {
            let _ = integrations::api::get_my_repositories().await;
        });
    };

    rsx! {
        main {
            components::sidebar::SideBar {},
            components::projects::Projects {},
            div {  onclick: click, class: "div-to-align-the-layout", "HEEEEELLLOOOOO WORLLLD" },
        }
    }
}
