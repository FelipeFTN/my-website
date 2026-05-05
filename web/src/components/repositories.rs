#![allow(non_snake_case)]
#![allow(unused_imports)]

use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use crate::integrations::my_api::*;

static STAR_ICON: Asset = asset!("/assets/star.svg");
static FORK_ICON: Asset = asset!("/assets/fork.svg");

#[component]
pub fn Repositories() -> Element {
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
                                name: "Failed to load".to_string(),
                                repo_owner: "NoOwner".to_string(),
                                description: "Failed to get repositories".to_string(),
                                stargazers_count: 0,
                                forks_count: 0,
                                language: "NoLanguage".to_string(),
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
                                name: "Failed to load".to_string(),
                                repo_owner: "NoOwner".to_string(),
                                description: "Failed to get repositories".to_string(),
                                stargazers_count: 0,
                                forks_count: 0,
                                language: "NoLanguage".to_string(),
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
        div { class: "repositories fade-in",
            h1 { class: "repos-title", "Repositories" }
            p { class: "repos-subtitle", "My GitHub repositories" }
            div { class: "repos-grid",
                {
                    repositories.iter().map(|repo| rsx!{
                        RepoItem { repo: repo.clone() }
                    })
                }
            }
            p { class: "repos-subtitle", "Projects I've contributed to" }
            div { class: "repos-grid",
                {
                    contr_repositories.iter().map(|repo| rsx!{
                        RepoItem { repo: repo.clone() }
                    })
                }
            }
        }
    }
}

#[component]
fn RepoItem(repo: MyRepositoriesData) -> Element {
    let url = format!("https://github.com/{}/{}", repo.repo_owner, repo.name);
    let language_url = format!("https://github.com/topics/{}", repo.language.to_lowercase().replace("+", "p"));
    let stars_url = format!("https://github.com/{}/{}/stargazers", repo.repo_owner, repo.name);
    let forks_url = format!("https://github.com/{}/{}/forks", repo.repo_owner, repo.name);
    rsx! {
        div { class: "repo-item",
            a { href: "{url}", target: "_blank", rel: "noopener noreferrer",
                h2 { class: "repo-name", "{repo.name}" }
            }
            p { class: "repo-desc", "{repo.description}" }
            div { class: "repo-tags",
                a { class: "repo-language", href: "{language_url}", target: "_blank", rel: "noopener noreferrer",
                    div { class: "repo-language-color {repo.language.to_string().replace(\"+\", \"Plus\")}" }
                    span { class: "repo-language-text", "{repo.language}" }
                }
                a { class: "repo-stars", href: "{stars_url}", target: "_blank", rel: "noopener noreferrer",
                    img { class: "repo-stars-icon", src: STAR_ICON, alt: "Stars" }
                    span { class: "repo-stars-text", "{repo.stargazers_count}" }
                }
                a { class: "repo-forks", href: "{forks_url}", target: "_blank", rel: "noopener noreferrer",
                    img { class: "repo-forks-icon", src: FORK_ICON, alt: "Forks" }
                    span { class: "repo-forks-text", "{repo.forks_count}" }
                }
            }
        }
    }
}
