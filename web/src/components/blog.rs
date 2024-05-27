#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        div { class: "blog",
            h1 { class: "title", "Blog" }
            p { class: "subtitle", "Here I post some of my thoughts..." }
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
