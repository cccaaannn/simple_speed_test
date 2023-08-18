use std::io;

use speed_test::*;

pub fn main() {
    const CONFIG_FILE_PATH: &str = "config/config.toml";
    let app_config: AppConfig = AppConfig::new(CONFIG_FILE_PATH);

    // Init logger
    simple_logger::init_with_level(app_config.log_level).unwrap();

    // Run speed test
    start_test(&app_config);

    println!("\nPress any key to exit\n");
    io::stdin().read_line(&mut String::new()).unwrap();
}
