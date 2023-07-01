use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub iteration: i32,
    pub download_urls: Vec<String>,
}

pub fn get_app_config() -> AppConfig {
    let conf_str: String =
        fs::read_to_string("config/config.toml").expect("Can not read config file");
    let app_config: AppConfig = toml::from_str(&conf_str).expect("Can not read config file");
    app_config
}
