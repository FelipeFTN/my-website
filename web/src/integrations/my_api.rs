#![allow(non_snake_case)]
// #![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use dioxus_logger::tracing::{Level, info, error};

// I don't know how I'll change this
// when I launch it online, but fuck it.
const API_HOST: &str = "http://127.0.0.1";
const API_PORT: &str = "8081";

#[derive(Serialize, Deserialize)]
pub struct MyRepositories {
    pub Name: String,
    pub Description: String,
    pub StargazersCount: i16, // I could use 8 bytes integer size
    pub ForksCount: i16,      // But the max size is 127, who knows I get big...
}

pub async fn get_my_repositories() -> Result<Vec<MyRepositories>> {
    let data = reqwest::get(format!("{:?}:{:?}", API_HOST, API_PORT))
        .await
        .expect("failed to reach MyAPI")
        .text()
        .await
        .expect("failed to get my repositories response");
        

    info!(data);

    let resp: Vec<MyRepositories> = serde_json::from_str(&data)
        .expect("failed to Deserialize my repositories response");

    Ok(resp)
}
