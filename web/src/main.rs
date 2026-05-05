#![allow(non_snake_case)]
#![allow(unused_imports)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

mod integrations;
mod components;

static CSS: Asset = asset!("/assets/styles/main.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    About {},
    #[route("/repositories")]
    Repositories {},
    #[route("/blog")]
    BlogList {},
    #[route("/blog/:slug")]
    BlogPost { slug: String },
    #[route("/games")]
    GamesPage {},
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

#[component]
fn About() -> Element {
    rsx! {
        components::navbar::NavBar {}
        div { class: "page-content",
            components::about::About {}
        }
    }
}

#[component]
fn Repositories() -> Element {
    rsx! {
        components::navbar::NavBar {}
        div { class: "page-content",
            components::repositories::Repositories {}
        }
    }
}

#[component]
fn BlogList() -> Element {
    rsx! {
        components::navbar::NavBar {}
        div { class: "page-content",
            components::blog::BlogList {}
        }
    }
}

#[component]
fn BlogPost(slug: String) -> Element {
    rsx! {
        components::navbar::NavBar {}
        div { class: "page-content",
            components::blog::BlogPost { slug }
        }
    }
}

#[component]
fn GamesPage() -> Element {
    rsx! {
        components::navbar::NavBar {}
        div { class: "page-content",
            components::games::GamesContent {}
        }
    }
}
