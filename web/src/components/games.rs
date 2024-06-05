#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Games() -> Element {
    rsx! {
        div { class: "games",
            h1 { class: "title", "Games" }
            p { class: "subtitle", "Top best games I have played..." }
        }
    }
}

fn Game() -> Element {
    rsx! {
        div { class: "game",
            h2 { class: "title", "Game Title" },
            p { class: "content", "Game Content" }
        }
    }
}
