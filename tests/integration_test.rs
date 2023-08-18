use speed_test::{start_test, AppConfig};

#[test]
fn should_read_config_file_and_perform_speed_test() {
    const CONFIG_FILE_PATH: &str = "config/config.toml";
    let app_config: AppConfig = AppConfig::new(CONFIG_FILE_PATH);
    start_test(&app_config);
}
