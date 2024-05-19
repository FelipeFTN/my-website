use config::Config;
use std::collections::HashMap;

pub struct AppConfig {
    pub cfg: Config,
} 

impl AppConfig {
    pub fn new() -> Self {
        let cfg = Config::builder()
            // Add in `./Settings.toml`
            .add_source(config::File::with_name("src/config/config"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            // .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        AppConfig { cfg }
    }

    pub fn get(&self, key: &str) -> String {
        self.cfg
            .clone()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap().get(key).unwrap().to_string()
    }
}
