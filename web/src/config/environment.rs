#![allow(non_snake_case)]

const _CONFIG: &str = include_str!("./config.toml");

// use std::collections::HashMap;
// use toml::Value;

// pub fn init() -> Result<(), String> {
//     let config: HashMap<String, Value> = toml::from_str(_CONFIG).expect("failed to parse config.toml");
//
//     for (key, value) in config {
//         match value {
//             Value::String(val) => {
//                 std::env::set_var(key, val);
//             }
//             _ => {
//                 return Err("only string values are supported".to_string());
//             }
//         }
//     }
//
//     Ok(())
// }

#[allow(dead_code)]
// I hate this shit, but I won't be able to use environment variables for now.
pub fn get(key: &str) -> Result<String, String> {
    match std::env::var(key) {
        Ok(val) => Ok(val),
        Err(_) => Err(format!("{} not found", key)),
    }
}
