use log::{error, Level};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct AppConfigFile {
    iteration: i32,
    download_urls: Vec<String>,
    log_level: String,
}

#[derive(Debug)]
pub struct AppConfig {
    pub iteration: i32,
    pub download_urls: Vec<String>,
    pub log_level: Level,
}

lazy_static! {
    pub static ref APP_CONFIG: AppConfig = get_app_config();
}

fn get_app_config() -> AppConfig {
    let conf_str: String = fs::read_to_string("config/config.toml")
        .map_err(|_| error!("Can not read config file"))
        .unwrap();
    let app_config_file: AppConfigFile = toml::from_str(&conf_str)
        .map_err(|_| error!("Can not parse config file"))
        .unwrap();

    let level: Level = match app_config_file.log_level.as_str() {
        "Error" => Level::Error,
        "Warn" => Level::Warn,
        "Info" => Level::Info,
        "Debug" => Level::Debug,
        "Trace" => Level::Trace,
        _ => Level::Info,
    };

    let app_config: AppConfig = AppConfig {
        iteration: app_config_file.iteration,
        download_urls: app_config_file.download_urls,
        log_level: level,
    };

    app_config
}
