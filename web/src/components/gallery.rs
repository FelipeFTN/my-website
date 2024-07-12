#![allow(non_snake_case)]

use dioxus::prelude::*;
use include_dir::{include_dir, Dir};
use dioxus_logger::tracing::{Level, info, error};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
struct Gallery {
    title: String,
    photo: String,
}

static GALLERY: Dir<'_> = include_dir!("assets/gallery/");

fn get_gallery() -> Vec<Gallery> {
    let gallery_count = GALLERY.files().count();
    if gallery_count == 0 {
        return vec![];
    }

    let mut gallery = Vec::with_capacity(gallery_count);
    for file in GALLERY.files() {
        let title = file.path().file_stem().unwrap().to_str().unwrap().to_string();
        let photo = format!("gallery/{}", file.path().to_str().unwrap().to_string());
        // info!("Gallery: {:?}", title);
        gallery.push(Gallery { title, photo });
    };

    gallery
}


#[component]
pub fn Gallery() -> Element {
    let gallery: Vec<Gallery> = get_gallery();
    let mut gallery_grid: Vec<Vec<Gallery>> = vec![];

    for i in (0..gallery.len()).step_by(2) {
        let end = std::cmp::min(i + 2, gallery.len());
        let gallery_row: Vec<Gallery> = gallery[i..end].to_vec();
        info!("index: {}; row: {:?}", i, gallery_row);

        // Check if the vector is not empty
        if !gallery_row.last().is_none() {
            gallery_grid.push(gallery_row);
        }
        // info!("{:?}", gallery_list);
    }
    info!("grid: {:?}", gallery_grid);

    rsx! {
        div { class: "gallery",
            h1 { class: "title", "Gallery" }
            p { class: "subtitle", "Here is my personal gallery..." }

            div { class: "gallery-grid",
                {
                    gallery_grid.iter().map(|g: &Vec<Gallery>| rsx!{
                        GalleryRow { gallery_list: g.clone() }
                    })
                }
            }
        }
    }
}

#[component]
fn GalleryRow(gallery_list: Vec<Gallery>) -> Element {
    rsx! {
        div { class: "gallery-row",
            {
                gallery_list.iter().map(|gallery: &Gallery| rsx!{
                    // Bro, I really hate using html stuff on dioxus components
                    // I was really destroyed at programming this time.
                    // I will come back here later, when I get less dump at Rust.
                    GalleryItem {
                        title: gallery.title.clone(),
                        photo: gallery.photo.clone(),
                        // onmousedown: move |_| {
                        //     selected_picture.set(Gallery{
                        //         title: gallery.title.clone(),
                        //         photo: gallery.photo.clone(),
                        //     });
                        // }
                    }
                })
            }
        }
    }
}

#[component]
fn GalleryItem(title: String, photo: String/* , onmousedown: EventHandler<MouseEvent> */) -> Element {
    let mut additional_classes = String::new();
    if title.contains("8") {
        additional_classes = "large".to_string();
    }

    // Here I'll do some symbols translations
    let f_title = title
        .replace("_", " ")
        .replace("9", "?")
        .replace("0", ".")
        .replace("1", "' ")
        .replace("2", ",")
        .replace("8", "")
    ; // is it ugly to do this with the semicolon?

    // Simple as f&ck
    rsx! {
        div { class: "frame",
            img { class: "photo {additional_classes}", src: photo },
            h2 { class: "title", "{f_title}" },
        }
    }
}
