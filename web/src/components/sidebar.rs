#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn SideBar() -> Element {
    rsx! {
        div { class: "sidebar",
            div { class: "profile",
                div { class: "profile-frame", 
                    img { src: "https://avatars.githubusercontent.com/u/80127749?v=4", alt: "Profile Picture"},
                }
                h1 { "Felipe Tenório" },
                p { "Software Developer" },
                div { class: "social-links", 
                    a { href: "https://github.com/FelipeFTN", 
                        img { src: "github.svg", alt: "Github", id: "github" }
                        span { "@FelipeFTN" }
                    },
                    a { href: "mailto:FelipeFTN@protonmail.com",
                        div { id: "email" }
                        span { "FelipeFTN@protonmail.com" }
                    }
                }
            }
        }
    }
}
