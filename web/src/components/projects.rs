#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info, error};

use crate::integrations::my_api::{MyRepositories, get_my_repositories};

const API_HOST: &str = "http://127.0.0.1";
const API_PORT: &str = "8081";

#[component]
pub fn Projects() -> Element {

    let mut repositories = use_signal(|| String::from("Loading..."));

    let fut = move |_| {
        spawn(async move {
            let data = reqwest::get("http://localhost:8081/v1/my_repositories")
                .await
                .expect("failed to reach MyAPI")
                .text()
                .await
                .expect("failed to get my repositories response");

            info!(data);
            repositories.set("Hello, World!".to_string());

            // let resp: Vec<MyRepositories> = serde_json::from_str(&data)
            //     .expect("failed to Deserialize my repositories response");
        });
    };

    rsx! {
        div { class: "projects",
            button { onclick: fut, "{repositories}" },
            h1 { class: "title", "Projects" },
            p { class: "subtitle", "GitHub repositories I've built."}
            div { class: "projects-grid",
                Item {},
                Item {},
                Item {},
                Item {},
                Item {},
                Item {},
                Item {},
            }
        }
    }
}

fn Item(/* repo: MyRepositories */) -> Element {
    rsx! {
        div { class: "project-item",
            h1 { "Project Title" },
            p { "Project Description" },
            div { class: "project-tags",
                a { class: "repo-language", href: "#",
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
