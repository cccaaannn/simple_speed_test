use log::{error, Level};
use serde::Deserialize;
use std::fs;
use std::process;

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

impl AppConfig {
    pub fn new() -> AppConfig {
        let conf_str: String = fs::read_to_string("config/config.toml").unwrap_or_else(|_| {
            error!("Can not read config file");
            process::exit(1)
        });

        let app_config_file: AppConfigFile = toml::from_str(&conf_str).unwrap_or_else(|_| {
            error!("Can not parse config file");
            process::exit(1)
        });

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
}
