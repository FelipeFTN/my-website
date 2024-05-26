#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info, error};

use crate::integrations::my_api::{*};

#[derive(Props, Clone, PartialEq)]
struct ItemProps {
    repo: MyRepositoriesData,
}

#[component]
pub fn Projects() -> Element {
    let mut repositories = use_signal(|| vec![]);
    let mut contr_repositories = use_signal(|| vec![]);

    let repos = move |_| {
        spawn(async move {
            let resp = get_my_repositories()
                .await
                .unwrap_or_else(|e| {
                    error!("failed to get my repositories: {:?}", e);
                    MyRepositories {
                        message: "failed to get my repositories".to_string(),
                        status: false,
                        data: vec![
                            MyRepositoriesData {
                                name: "Failed to get my repositories".to_string(),
                                description: "Failed to get my repositories".to_string(),
                                stargazers_count: 0,
                                forks_count: 0,
                            }
                        ],
                    }
                });

            repositories.set(resp.data);
        });
    };
    if repositories.len() == 0 {
        repos("Fetch repositories");
    }

    let contr_repos = move |_| {
        spawn(async move {
            let resp = get_contributed_repositories()
                .await
                .unwrap_or_else(|e| {
                    error!("failed to get contributed repositories: {:?}", e);
                    MyRepositories {
                        message: "failed to get contributed repositories".to_string(),
                        status: false,
                        data: vec![
                            MyRepositoriesData {
                                name: "Failed to get contributed repositories".to_string(),
                                description: "Failed to get contributed repositories".to_string(),
                                stargazers_count: 0,
                                forks_count: 0,
                            }
                        ],
                    }
                });

            contr_repositories.set(resp.data);
        });
    };
    if contr_repositories.len() == 0 {
        contr_repos("Fetch contributed repositories");
    }

    rsx! {
        div { class: "projects",
            h1 { class: "title", "Projects" },
            p { class: "subtitle", "GitHub repositories I've built."},
            div { class: "projects-grid",
                {
                    repositories.iter().map(|repo| rsx!{
                        Item { repo: repo.clone() }
                    }).collect::<Vec<_>>().into_iter()  // Collect the iterator to a vector
                },
            },
            p { class: "subtitle", "Projects I collaborated on."},
            div { class: "projects-grid",
                {
                    contr_repositories.iter().map(|repo| rsx!{
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
