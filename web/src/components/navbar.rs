#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    let route: Route = use_route::<Route>();
    let blog_active = matches!(route, Route::BlogList {} | Route::BlogPost { .. });

    rsx! {
        nav { class: "navbar",
            div { class: "navbar-container",
                div { class: "navbar-brand",
                    Link { to: Route::About {},
                        div { class: "brand-content",
                            img {
                                src: "https://avatars.githubusercontent.com/u/80127749?v=4",
                                class: "brand-avatar",
                                alt: "Felipe Tenório"
                            }
                            div {
                                span { class: "brand-name", "Felipe Tenório" }
                                span { class: "brand-role", "Software Engineer" }
                            }
                        }
                    }
                }
                ul { class: "navbar-links",
                    li {
                        Link {
                            to: Route::About {},
                            class: "nav-link",
                            active_class: "nav-active",
                            "About"
                        }
                    }
                    li {
                        Link {
                            to: Route::Repositories {},
                            class: "nav-link",
                            active_class: "nav-active",
                            "Repositories"
                        }
                    }
                    li {
                        Link {
                            to: Route::BlogList {},
                            class: if blog_active { "nav-link nav-active" } else { "nav-link" },
                            "Blog"
                        }
                    }
                    li {
                        Link {
                            to: Route::GamesPage {},
                            class: "nav-link",
                            active_class: "nav-active",
                            "Games"
                        }
                    }
                }
            }
        }
    }
}
