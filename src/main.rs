use std::io;

use crate::config_utils::AppConfig;

mod config_utils;
mod speed_test;

pub fn main() {

    const CONFIG_FILE_PATH: &str = "config/config.toml";
    let app_config: AppConfig = AppConfig::new(CONFIG_FILE_PATH);

    // Init logger
    simple_logger::init_with_level(app_config.log_level).unwrap();

    // Run speed test
    speed_test::start(&app_config);

    println!("\nPress any key to exit\n");
    io::stdin().read_line(&mut String::new()).unwrap();
}
