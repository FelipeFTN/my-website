#![allow(non_snake_case)]

use dioxus::prelude::*;

struct Game {
    title: String,
    cover: String,
}

fn get_games() -> Vec<Game> {
    vec![
        Game {
            title: "Bioshock".to_string(),
            cover: "games/bioshock.png".to_string(),
            
        },
        Game {
            title: "Age of Empires II".to_string(),
            cover: "games/age_of_empires_ii.png".to_string(),
        },
        Game {
            title: "The Witcher".to_string(),
            cover: "games/the_witcher.png".to_string(),
        },
        Game {
            title: "Portal".to_string(),
            cover: "games/portal.png".to_string(),
        },
        Game {
            title: "Amnesia: The Dark Descent".to_string(),
            cover: "games/amnesia.png".to_string(),
        },
        Game {
            title: "Fallout 3".to_string(),
            cover: "games/fallout.png".to_string(),
        },
        Game {
            title: "Grim Fandango".to_string(),
            cover: "games/grim_fandango.png".to_string(),
        },
        Game {
            title: "Cyberpunk 2077".to_string(),
            cover: "games/cyberpunk_2077.webp".to_string(),
        },
        Game {
            title: "Half-Life".to_string(),
            cover: "games/half_life.png".to_string(),
        },
        Game {
            title: "The Elder Scrolls IV: Oblivion".to_string(),
            cover: "games/the_elder_scrolls_oblivion.png".to_string(),
        },
        Game {
            title: "Assassin's Creed II".to_string(),
            cover: "games/assassins_creed_ii.png".to_string(),
        },
        Game {
            title: "Warcraft III".to_string(),
            cover: "games/warcraft_iii.png".to_string(),
        },
    ]
}


#[component]
pub fn Games() -> Element {
    rsx! {
        div { class: "games",
            h1 { class: "title", "Games" }
            p { class: "subtitle", "Top best games I have played..." }

            div { class: "games-list",
                {
                    get_games().iter().map(|game| rsx!{
                        GameItem { title: game.title.clone(), cover: game.cover.clone() }
                    }).collect::<Vec<_>>().into_iter()
                }
            }
        },
    }
}

#[component]
fn GameItem(title: String, cover: String) -> Element {
    rsx! {
        div { class: "game",
            div { class: "game-wrapper",
                // h2 { class: "title", title },
                img { class: "cover", src: cover }
                div { class: "gradient" }
            }
        }
    }
}
