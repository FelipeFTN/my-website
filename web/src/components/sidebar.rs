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
                    a { href: "mailto:FelipeFTN@protonmail.com",
                    target: "_blank", rel: "noopener noreferrer",
                        div { id: "email" }
                        span { "FelipeFTN@protonmail.com" }
                    },
                    a { href: "https://github.com/FelipeFTN", 
                    target: "_blank", rel: "noopener noreferrer",
                        img { src: "github.svg", alt: "Github", id: "github" }
                        span { "@FelipeFTN" }
                    },
                    a { href: "https://www.instagram.com/_felipeftn",
                    target: "_blank", rel: "noopener noreferrer",
                        img { src: "instagram.svg", alt: "Instagram", id: "instagram" }
                        span { "@_felipeftn" }
                    },
                    a { href: "https://matrix.to/#/@felipeftn:matrix.org",
                    target: "_blank", rel: "noopener noreferrer",
                        img { src: "element.svg", alt: "Element", id: "element" }
                        span { "@felipeftn" }
                    },
                    a { href: "https://steamcommunity.com/id/FelipeFTN",
                    target: "_blank", rel: "noopener noreferrer",
                        img { src: "steam.svg", alt: "Steam", id: "steam" }
                        span { "FelipeFTN" }
                    },
                }
            }
        }
    }
}
