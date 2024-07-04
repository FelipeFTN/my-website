#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        div { class: "blog",
            h1 { class: "title", "Blog" }
            p { class: "subtitle", "Here I post some of my thoughts..." }
            div { class: "not-found",
                h1 { "I will be working on this section soon! ;)" }
            }
        }
    }
}

fn Post() -> Element {
    rsx! {
        div { class: "post",
            h2 { class: "title", "Post Title" },
            p { class: "content", "Post Content" }
        }
    }
}
