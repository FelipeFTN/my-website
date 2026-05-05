#![allow(non_snake_case)]

use dioxus::prelude::*;

static BIOSHOCK:    Asset = asset!("/assets/games/bioshock.png");
static AOE2:        Asset = asset!("/assets/games/age_of_empires_ii.png");
static WITCHER:     Asset = asset!("/assets/games/the_witcher.png");
static PORTAL:      Asset = asset!("/assets/games/portal.png");
static AMNESIA:     Asset = asset!("/assets/games/amnesia.png");
static FALLOUT:     Asset = asset!("/assets/games/fallout.png");
static GRIM:        Asset = asset!("/assets/games/grim_fandango.png");
static CYBERPUNK:   Asset = asset!("/assets/games/cyberpunk_2077.webp");
static HALF_LIFE:   Asset = asset!("/assets/games/half_life.png");
static OBLIVION:    Asset = asset!("/assets/games/the_elder_scrolls_oblivion.png");
static AC2:         Asset = asset!("/assets/games/assassins_creed_ii.png");
static WARCRAFT3:   Asset = asset!("/assets/games/warcraft_iii.png");

struct Game {
    title: &'static str,
    cover: Asset,
}

fn get_games() -> Vec<Game> {
    vec![
        Game { title: "Bioshock",                        cover: BIOSHOCK },
        Game { title: "Age of Empires II",               cover: AOE2 },
        Game { title: "The Witcher",                     cover: WITCHER },
        Game { title: "Portal",                          cover: PORTAL },
        Game { title: "Amnesia: The Dark Descent",       cover: AMNESIA },
        Game { title: "Fallout 3",                       cover: FALLOUT },
        Game { title: "Grim Fandango",                   cover: GRIM },
        Game { title: "Cyberpunk 2077",                  cover: CYBERPUNK },
        Game { title: "Half-Life",                       cover: HALF_LIFE },
        Game { title: "The Elder Scrolls IV: Oblivion",  cover: OBLIVION },
        Game { title: "Assassin's Creed II",             cover: AC2 },
        Game { title: "Warcraft III",                    cover: WARCRAFT3 },
    ]
}

#[component]
pub fn GamesContent() -> Element {
    rsx! {
        div { class: "games fade-in",
            h1 { class: "games-title", "Games" }
            p { class: "games-subtitle", "Top games I have played." }
            div { class: "games-list",
                {get_games().into_iter().map(|game| rsx!{
                    GameItem { title: game.title, cover: game.cover }
                })}
            }
        }
    }
}

#[component]
fn GameItem(title: &'static str, cover: Asset) -> Element {
    rsx! {
        div { class: "game",
            div { class: "game-wrapper",
                img { class: "cover", src: "{cover}", alt: "{title}" }
                div { class: "gradient" }
            }
        }
    }
}
