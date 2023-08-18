use owo_colors::OwoColorize;
use reqwest::blocking::get;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crate::config_utils::AppConfig;

struct PerSeconds {
    bps: f64,
    kbps: f64,
    mbps: f64,
}

fn calculate_per_seconds(content_size_bits: f64, duration: f64) -> PerSeconds {
    let bps: f64 = content_size_bits / duration;
    let kbps: f64 = bps / 1024.0;
    let mbps: f64 = kbps / 1024.0;
    return PerSeconds { bps, kbps, mbps };
}

fn download_content(download_url: String) -> PerSeconds {
    let start: Instant = Instant::now();

    let resp = get(download_url).expect("Can not fetch file");

    let headers = resp.headers().clone();
    let _ = resp.text();
    let duration: Duration = start.elapsed();

    let content_length_str: &str = headers
        .get("Content-length")
        .expect("Can not get Content-length")
        .to_str()
        .expect("Can not convert Content-length to string");

    let content_size_bits: f64 = content_length_str
        .parse::<f64>()
        .expect("Can not convert Content-length to numeric value")
        * 8.0;

    return calculate_per_seconds(content_size_bits, duration.as_secs_f64());
}

pub fn start_test(app_config: &AppConfig) {
    let mut total_bps: f64 = 0.0;

    for iteration in 0..app_config.iteration {
        println!("Iteration: {}", iteration + 1);
        for download_url in &app_config.download_urls {
            print!("Downloading: {}", download_url.blue());
            let _ = stdout().flush();
            let per_seconds: PerSeconds = download_content(download_url.to_owned());
            total_bps += per_seconds.bps;
            println!(" [Mbps: {:.*}]", 3, per_seconds.mbps.green());
        }
        println!();
    }

    let per_seconds: PerSeconds = calculate_per_seconds(
        total_bps,
        (app_config.download_urls.len() as i32 * app_config.iteration) as f64,
    );

    println!("Kbps: {:.*}", 3, per_seconds.kbps.green());
    println!("Mbps: {:.*}", 3, per_seconds.mbps.green());
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[test]
    #[should_panic(expected = "Can not fetch file")]
    fn should_panic_on_download_error() {
        download_content("".to_owned());
    }

    #[test]
    fn should_calculate_per_seconds_from_download() {
        let mut server = mockito::Server::new();

        let url = server.url();

        // Create a mock
        let _mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("test")
            .expect(1)
            .create();

        let per_seconds: PerSeconds = download_content(url.to_owned());

        _mock.assert();
        assert!(per_seconds.bps > 0 as f64);
        assert!(per_seconds.kbps > 0 as f64);
        assert!(per_seconds.mbps > 0 as f64);
    }

    #[test]
    fn should_calculate_per_seconds() {
        let per_seconds: PerSeconds = calculate_per_seconds(1_048_576.0, 1.0);
        assert_eq!(1_048_576.0, per_seconds.bps);
        assert_eq!(1024.0, per_seconds.kbps);
        assert_eq!(1.0, per_seconds.mbps);
    }

    #[test]
    #[should_panic(expected = "Can not fetch file")]
    fn should_fail_to_download() {
        let wrong_download_url: &str = "test";
        let app_config: AppConfig = AppConfig {
            iteration: 1,
            download_urls: vec![wrong_download_url.to_string()],
            log_level: log::Level::Info,
        };
        start_test(&app_config);
    }

    #[test]
    fn should_perform_download_and_calculate_per_seconds() {
        let mut server = mockito::Server::new();

        let url = server.url();

        // Create a mock
        let _mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("test")
            .expect(2)
            .create();

        let app_config: AppConfig = AppConfig {
            iteration: 2,
            download_urls: vec![url],
            log_level: log::Level::Info,
        };

        start_test(&app_config);

        _mock.assert();
    }
}
