#![allow(non_snake_case)]

use dioxus::prelude::*;
use pulldown_cmark::{Parser, Options, html};
use crate::Route;

pub struct PostMeta {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub read_time: String,
}

pub fn post_metadata() -> Vec<PostMeta> {
    vec![
        PostMeta {
            slug: "game-memory-hacking".to_string(),
            title: "Cheating the Stack: A Developer's Guide to Game Memory Hacking".to_string(),
            date: "2025-01-15".to_string(),
            description: "Dive into process memory, virtual address spaces, and how tools like Cheat Engine work under the hood. Includes a C memory scanner implementation.".to_string(),
            read_time: "8 min read".to_string(),
        },
        PostMeta {
            slug: "firmware-hardware-hacking".to_string(),
            title: "From Silicon to Shell: Hardware & Firmware Hacking Fundamentals".to_string(),
            date: "2025-01-08".to_string(),
            description: "UART, JTAG, SPI flash — how to extract and reverse engineer firmware from embedded devices. Practical guide with real tooling.".to_string(),
            read_time: "9 min read".to_string(),
        },
        PostMeta {
            slug: "neovim-arch-linux".to_string(),
            title: "My Neovim + Arch Linux Setup: A Minimalist Dev Environment".to_string(),
            date: "2024-12-20".to_string(),
            description: "Why Arch Linux, how I manage dotfiles, my Neovim Lua config, key plugins, LSP setup for Rust/Go, and the tiling WM workflow.".to_string(),
            read_time: "7 min read".to_string(),
        },
        PostMeta {
            slug: "pipe-organ-simulator".to_string(),
            title: "Building a Physical Pipe Organ Interface with Arduino and a Shift Register".to_string(),
            date: "2025-05-05".to_string(),
            description: "Connecting a real 61-key organ manual to GrandOrgue and RustyPipes using an Arduino Pro Micro (ATmega32u4) and 74HC165 shift registers over USB MIDI.".to_string(),
            read_time: "10 min read".to_string(),
        },
    ]
}

pub fn get_post_content(slug: &str) -> Option<&'static str> {
    match slug {
        "game-memory-hacking" => Some(include_str!("../../assets/blog/game-memory-hacking.md")),
        "firmware-hardware-hacking" => Some(include_str!("../../assets/blog/firmware-hardware-hacking.md")),
        "neovim-arch-linux" => Some(include_str!("../../assets/blog/neovim-arch-linux.md")),
        "pipe-organ-simulator" => Some(include_str!("../../assets/blog/pipe-organ-simulator.md")),
        _ => None,
    }
}

pub fn markdown_to_html(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[component]
pub fn BlogList() -> Element {
    let posts = post_metadata();
    rsx! {
        div { class: "blog-list fade-in",
            div { class: "blog-list-header",
                h1 { "Blog" }
                p { "Thoughts on systems programming, low-level computing, and the tools I use." }
            }
            div { class: "post-cards",
                {posts.into_iter().map(|post| {
                    let slug = post.slug.clone();
                    rsx! {
                        Link {
                            to: Route::BlogPost { slug: slug },
                            class: "post-card",
                            div { class: "post-card-meta",
                                span { class: "post-date", "{post.date}" }
                                span { class: "post-read-time", "{post.read_time}" }
                            }
                            div { class: "post-title", "{post.title}" }
                            div { class: "post-description", "{post.description}" }
                            div { class: "post-read-more", "Read →" }
                        }
                    }
                })}
            }
        }
    }
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    let posts = post_metadata();
    let meta = posts.into_iter().find(|p| p.slug == slug);

    match (meta, get_post_content(&slug)) {
        (Some(post), Some(content)) => {
            let html_content = markdown_to_html(content);
            rsx! {
                div { class: "blog-post fade-in",
                    Link { to: Route::BlogList {}, class: "post-back",
                        "← Back to Blog"
                    }
                    div { class: "post-header",
                        h1 { "{post.title}" }
                        div { class: "post-header-meta",
                            span { class: "post-date", "{post.date}" }
                            span { class: "post-read-time", "{post.read_time}" }
                        }
                    }
                    div {
                        class: "markdown-content",
                        dangerous_inner_html: "{html_content}"
                    }
                }
            }
        }
        _ => {
            rsx! {
                div { class: "post-not-found fade-in",
                    h2 { "Post not found" }
                    p { "The post you're looking for doesn't exist." }
                    Link { to: Route::BlogList {}, "← Back to Blog" }
                }
            }
        }
    }
}
