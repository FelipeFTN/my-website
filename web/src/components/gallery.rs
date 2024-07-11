#![allow(non_snake_case)]

use dioxus::prelude::*;
use include_dir::{include_dir, Dir};
use dioxus_logger::tracing::{Level, info, error};
use std::path::Path;

#[derive(Debug)]
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
        info!("Gallery: {:?}", title);
        gallery.push(Gallery { title, photo });
    };

    gallery
}


#[component]
pub fn Gallery() -> Element {
    // let mut selected_picture = use_signal(|| Gallery{title: "".to_string(), photo: "".to_string()});

    rsx! {
        div { class: "gallery",
            h1 { class: "title", "Gallery" }
            p { class: "subtitle", "Here is my personal gallery..." }

            div { class: "gallery-grid",
                {
                    get_gallery().iter().map(|gallery: &Gallery| rsx!{
                        GalleryItem {
                            title: gallery.title.clone(), photo: gallery.photo.clone(),
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
}

#[component]
fn GalleryItem(title: String, photo: String/* , onmousedown: EventHandler<MouseEvent> */) -> Element {
    rsx! {
        div { class: "frame",
            div { class: "frame-wrapper",
                h2 { class: "title", title },
                img { class: "photo", src: photo }
                div { class: "gradient" }
            }
        }
    }
}
