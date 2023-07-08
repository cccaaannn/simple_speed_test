#[macro_use]
extern crate lazy_static;

mod config_utils;
mod speed_test;

fn main() {
    // Init logger
    simple_logger::init_with_level(config_utils::APP_CONFIG.log_level).unwrap();

    // Run speed test
    speed_test::start();
}
