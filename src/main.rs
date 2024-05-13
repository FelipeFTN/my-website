#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

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
        SideBar {},
    }
}

#[component]
fn SideBar() -> Element {
    rsx! {
        div { class: "sidebar",
            div { class: "profile",
                img { src: "https://avatars.githubusercontent.com/u/80127749?v=4", alt: "Profile Picture"},
                h1 { "Felipe Tenório" },
                p { "Software Developer" },
            }
        }
    }
}
