#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn SideBar() -> Element {
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
