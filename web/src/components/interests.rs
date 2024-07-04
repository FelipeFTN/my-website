#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Interests() -> Element {
    rsx! {
        div { class: "interests",
            h1 { class: "title", "Interests" }
            p { class: "subtitle", "Topics that I want to learn about." }
            div { class: "not-found",
                h1 { "I will be working on this section soon! ;)" }
            }
        }
    }
}

fn Book(title: String, image: String) -> Element {
    rsx! {
        div { class: "book",
            h2 { class: "title", "Book Title" },
            p { class: "content", "Book Content" }
        }
    }
}
