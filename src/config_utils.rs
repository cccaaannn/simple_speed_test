use log::Level;
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

impl AppConfig {
    pub fn new(file_path: &str) -> AppConfig {
        let conf_str: String =
            fs::read_to_string(file_path.to_string()).expect("Can not read config file");

        let app_config_file: AppConfigFile =
            toml::from_str(&conf_str).expect("Can not parse config file");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Can not read config file")]
    fn should_fail_to_read_config_file() {
        const CONFIG_FILE_PATH: &str = "";
        AppConfig::new(CONFIG_FILE_PATH);
    }

    #[test]
    fn should_read_config_file() {
        const CONFIG_FILE_PATH: &str = "config/config.toml";
        let app_config: AppConfig = AppConfig::new(CONFIG_FILE_PATH);

        assert_ne!(app_config.iteration, 0);
        assert_ne!(app_config.download_urls.len(), 0);
    }

    #[test]
    fn should_test_empty_log_level() {
        const CONFIG_FILE_PATH: &str = "tests/resources/config/log_levels/_config.toml";
        let app_config: AppConfig = AppConfig::new(CONFIG_FILE_PATH);

        assert_eq!(app_config.iteration, 1);
        assert_eq!(app_config.download_urls.len(), 0);
        assert_eq!(app_config.log_level, Level::Info);
    }
}
