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
                    // I really need to align this shit better later.
                    a { href: "https://github.com/FelipeFTN", 
                        img { src: "github.svg", alt: "Github" }
                        span { "@FelipeFTN" }
                    },
                }
            }
        }
    }
}
