#![allow(non_snake_case)]
// #![allow(dead_code)]

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use dioxus_logger::tracing::{Level, info, error};

// I don't know how I'll change this
// when I launch it online, but fuck it.
// const API_HOST: &str = "http://127.0.0.1";
// const API_PORT: &str = "8081";

// I really should put this into some entities place
#[derive(Props, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyRepositories {
    pub message: String,
    pub status: bool,
    pub data: Vec<MyRepositoriesData>,
}

#[derive(Props, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyRepositoriesData {
    pub name: String,
    pub repo_owner: String,
    pub description: String,
    pub stargazers_count: i16, // I could use 8 bytes integer size
    pub forks_count: i16,      // But the max size is 127, who knows I get big...
    pub language: String,
}

pub async fn get_my_repositories() -> Result<MyRepositories> {
    let response = reqwest::get("http://localhost:8081/v1/my_repositories")
        .await
        .expect("failed to reach MyAPI")
        .text()
        .await;

    let data = match response {
        Ok(resp) => resp,
        Err(_) => String::from("Failed to get response"),
    };
        
    // info!(data);
    let resp: MyRepositories = serde_json::from_str(&data)?; // propagating error with ? operator

    Ok(resp)
}

pub async fn get_contributed_repositories() -> Result<MyRepositories> {
    let response = reqwest::get("http://localhost:8081/v1/contributed_repositories")
        .await
        .expect("failed to reach MyAPI")
        .text()
        .await;

    let data = match response {
        Ok(resp) => resp,
        Err(_) => String::from("Failed to get response"),
    };

    // info!(data);
    let resp: MyRepositories = serde_json::from_str(&data)?; // propagating error with ? operator

    Ok(resp)
}
