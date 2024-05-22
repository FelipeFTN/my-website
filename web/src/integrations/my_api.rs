#![allow(non_snake_case)]
// #![allow(dead_code)]

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use dioxus_logger::tracing::{Level, info, error};

// I don't know how I'll change this
// when I launch it online, but fuck it.
const API_HOST: &str = "http://127.0.0.1";
const API_PORT: &str = "8081";

#[derive(Props, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyRepositories {
    pub message: String,
    pub status: bool,
    pub data: Vec<MyRepositoriesData>,
}

#[derive(Props, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyRepositoriesData {
    pub name: String,
    pub description: String,
    pub stargazers_count: i16, // I could use 8 bytes integer size
    pub forks_count: i16,      // But the max size is 127, who knows I get big...

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
