#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Books() -> Element {
    rsx! {
        div { class: "books",
            h1 { class: "title", "Books" }
            p { class: "subtitle", "Some of the books I've read..." }
        }
    }
}

fn Book() -> Element {
    rsx! {
        div { class: "book",
            h2 { class: "title", "Book Title" },
            p { class: "content", "Book Content" }
        }
    }
}
