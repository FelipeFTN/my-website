#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info, error};

use crate::integrations::my_api::{MyRepositories, MyRepositoriesData, get_my_repositories};

const API_HOST: &str = "http://127.0.0.1";
const API_PORT: &str = "8081";

#[derive(Props, Clone, PartialEq)]
struct ItemProps {
    repo: MyRepositoriesData,
}

#[component]
pub fn Projects() -> Element {

    let mut repositories = use_signal(|| vec![]);
    //     let repositories = vec![
    //     MyRepositories {
    //         Name: "dioxus".to_string(),
    //         Description: "A Rust web framework".to_string(),
    //         StargazersCount: 1000,
    //         ForksCount: 100,
    //     },
    //     // Add more repositories as needed
    // ];

    let fut = move |_| {
        spawn(async move {
            let data = reqwest::get("http://localhost:8081/v1/my_repositories")
                .await
                .expect("failed to reach MyAPI")
                .text()
                .await
                .expect("failed to get my repositories response");

            info!(data);
            let resp: MyRepositories = serde_json::from_str(&data)
                .expect("failed to Deserialize my repositories response");

            repositories.set(resp.data);
        });
    };

    rsx! {
        div { class: "projects",
            h1 { class: "title", "Projects" },
            p { class: "subtitle", "GitHub repositories I've built."},
            div { class: "projects-grid",
                {fut("Fetch Repositories")},
                {
                    repositories.iter().map(|repo| rsx!{
                        Item { repo: repo.clone() }
                    }).collect::<Vec<_>>().into_iter()  // Collect the iterator to a vector
                },
            },
        }
    }
}

fn Item(props: ItemProps) -> Element {
    rsx! {
        div { class: "project-item",
            h1 { "{props.repo.name}" },
            p { "{props.repo.description}" },
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
