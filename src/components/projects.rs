#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "projects",
            h1 { class: "title", "Projects" },
            p { class: "subtitle", "GitHub repositories I've built."}
            div { class: "projects-grid",
                // Here, we will do a for loop to render the projects
                Item{},
            }
        }
    }
}

fn Item() -> Element {
    rsx! {
        div { class: "project-item",
            h1 { "Project Title" },
            p { "Project Description" },
            div { class: "project-tags",
                a { class: "repo-language", href: "#", "Tag 1"
                    span { class: "repo-language-color", "*" },
                    span { class: "repo-language-text", "Language" },
                },
                a { class: "repo-stars", href: "#",
                    span { class: "repo-stars-icon", "*" },
                    span { class: "repo-stars-text", "Stars" },
                },
                a { class: "repo-forks", href: "#",
                    span { class: "repo-forks-icon", "*" },
                    span { class: "repo-forks-text", "Forks" },
                },
            }
        }
    }
}
