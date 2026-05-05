#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "about-page fade-in",
            // Hero
            div { class: "hero",
                img {
                    src: "https://avatars.githubusercontent.com/u/80127749?v=4",
                    class: "hero-avatar",
                    alt: "Felipe Tenório"
                }
                div { class: "hero-info",
                    h1 { "Felipe Tenório" }
                    p { class: "hero-title", "Software Engineer" }
                    div { class: "hero-social",
                        a {
                            href: "https://github.com/FelipeFTN",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "social-link",
                            "⌥ GitHub"
                        }
                        a {
                            href: "mailto:FelipeFTN@protonmail.com",
                            class: "social-link",
                            "✉ Email"
                        }
                        a {
                            href: "https://instagram.com/_felipeftn",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "social-link",
                            "◈ Instagram"
                        }
                        a {
                            href: "https://matrix.to/#/@felipeftn:matrix.org",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "social-link",
                            "⬡ Matrix"
                        }
                    }
                }
            }

            // About section
            section { class: "about-section",
                h2 { class: "section-title", "// about" }
                div { class: "about-text",
                    p {
                        "Software Engineer with a deep obsession for systems programming, low-level computing, and building "
                        "things that are fast, reliable, and elegant. I spend a lot of my time at the intersection of "
                        "hardware and software — from reading kernel source code and writing assembly routines to building "
                        "scalable cloud-native backends in Go and Rust."
                    }
                    p {
                        "I got into low-level stuff out of curiosity — wanted to understand what was actually happening under the abstractions. "
                        "That rabbit hole never really ended. Now I write assembly when I want to understand something, read kernel source when "
                        "I'm debugging weird behavior, and generally can't look at a C pointer without thinking about what the allocator is doing."
                    }
                    p {
                        "Active open-source contributor with accepted patches to dunst (C, lightweight notification daemon), "
                        "GNOME (modules and documentation), and various Linux projects. Advanced Arch Linux user — minimalist, "
                        "highly customized environment with tiling WMs, custom keybindings, and a Neovim setup that would make "
                        "your IDE jealous."
                    }
                    p {
                        "Specialized in high-performance back-end, cloud-native AWS architectures, and Data Engineering/ML "
                        "pipelines on Databricks (Spark, Delta Lake, MLflow). Looking for challenging roles in Software "
                        "Engineering, Data Engineering, or ML Engineering where performance and scalability matter."
                    }
                }
            }

            // Expertise section
            section { class: "expertise-section",
                h2 { class: "section-title", "// expertise" }
                div { class: "expertise-grid",
                    ExpertiseCard {
                        icon: "⚙️",
                        title: "Systems & Low-Level",
                        tags: vec![
                            "Assembly (x86/ARM)".to_string(), "C/C++".to_string(),
                            "Memory Management".to_string(), "Stack & Heap internals".to_string(),
                            "Kernel development".to_string(), "Process & Thread models".to_string(),
                            "ELF/PE binary formats".to_string(), "Reverse Engineering".to_string(),
                        ]
                    }
                    ExpertiseCard {
                        icon: "🦀",
                        title: "Languages",
                        tags: vec![
                            "Rust".to_string(), "Go".to_string(), "C/C++".to_string(),
                            "Zig".to_string(), "Python".to_string(), "TypeScript/JavaScript".to_string(),
                            "Java".to_string(), "Dart/Flutter".to_string(), "Assembly".to_string(),
                        ]
                    }
                    ExpertiseCard {
                        icon: "☁️",
                        title: "Cloud & Backend",
                        tags: vec![
                            "AWS (Lambda, DynamoDB, Aurora)".to_string(),
                            "ECS/EC2, S3, CloudFront".to_string(),
                            "API Gateway".to_string(), "Microservices".to_string(),
                            "REST/GraphQL".to_string(), "Clean Architecture".to_string(),
                            "TDD".to_string(), "CI/CD".to_string(),
                        ]
                    }
                    ExpertiseCard {
                        icon: "📊",
                        title: "Data & ML",
                        tags: vec![
                            "Databricks".to_string(), "Apache Spark".to_string(),
                            "Delta Live Tables".to_string(), "MLflow".to_string(),
                            "Pandas/NumPy/Scikit-learn".to_string(), "ETL pipelines".to_string(),
                            "Delta Lake".to_string(),
                        ]
                    }
                    ExpertiseCard {
                        icon: "🗄️",
                        title: "Databases",
                        tags: vec![
                            "MySQL".to_string(), "SQL Server".to_string(),
                            "DynamoDB".to_string(), "Aurora PostgreSQL".to_string(),
                        ]
                    }
                    ExpertiseCard {
                        icon: "🐧",
                        title: "Open Source & Linux",
                        tags: vec![
                            "Arch Linux".to_string(), "Neovim".to_string(),
                            "dunst contributor".to_string(), "GNOME contributor".to_string(),
                            "dotfiles enthusiast".to_string(), "tiling WMs".to_string(),
                        ]
                    }
                }
            }

            // Education section
            section { class: "education-section",
                h2 { class: "section-title", "// education" }
                div { class: "edu-list",
                    div { class: "edu-item",
                        div { class: "edu-dot edu-dot-red" }
                        div {
                            div { class: "edu-name", "CS50 — Harvard University" }
                            div { class: "edu-detail", "Introduction to Computer Science" }
                            div { class: "edu-status", "currently enrolled" }
                        }
                    }
                    div { class: "edu-item",
                        div { class: "edu-dot" }
                        div {
                            div { class: "edu-name", "CEAP" }
                            div { class: "edu-detail", "Technical Degree in Informatics" }
                        }
                    }
                    div { class: "edu-item",
                        div { class: "edu-dot" }
                        div {
                            div { class: "edu-name", "Univesp" }
                            div { class: "edu-detail", "Software Engineering" }
                        }
                    }
                    div { class: "edu-item",
                        div { class: "edu-dot" }
                        div {
                            div { class: "edu-name", "Kenzie Academy" }
                            div { class: "edu-detail", "Full Stack Development" }
                        }
                    }
                    div { class: "edu-item",
                        div { class: "edu-dot" }
                        div {
                            div { class: "edu-name", "Alura" }
                            div { class: "edu-detail", "Software Engineering" }
                        }
                    }
                }
            }

            // Open Source section
            section { class: "opensource-section",
                h2 { class: "section-title", "// open_source" }
                div { class: "oss-list",
                    div { class: "oss-item",
                        div { class: "oss-name", "dunst" }
                        div { class: "oss-desc",
                            "C lightweight notification daemon. Contributed bug fixes and feature improvements to the notification system."
                        }
                        a {
                            href: "https://github.com/dunst-project/dunst",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "oss-link",
                            "github.com/dunst-project/dunst"
                        }
                    }
                    div { class: "oss-item",
                        div { class: "oss-name", "GNOME" }
                        div { class: "oss-desc",
                            "Contributed to GNOME modules, documentation, and ecosystem tooling."
                        }
                        a {
                            href: "https://gitlab.gnome.org",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "oss-link",
                            "gitlab.gnome.org"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExpertiseCard(icon: String, title: String, tags: Vec<String>) -> Element {
    rsx! {
        div { class: "expertise-card",
            div { class: "card-header",
                span { class: "card-icon", "{icon}" }
                span { class: "card-title", "{title}" }
            }
            div { class: "card-tags",
                {tags.iter().map(|tag| rsx! {
                    span { class: "tag", "{tag}" }
                })}
            }
        }
    }
}
