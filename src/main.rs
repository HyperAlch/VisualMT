mod settings;

use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use xcap::Monitor;

const SETTINGS_ERROR_SPACE: isize = 100;

fn main() {
    // Load settings
    let mut settings = Settings::default();
    settings.load().unwrap();

    // Start XCap and save screenshots
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();
    let ocr_monitor = monitors
        .get(settings.ocr_monitor_number)
        .expect("Monitor not found");
    let icon_monitor = monitors
        .get(settings.icon_monitor_number)
        .expect("Monitor not found");

    if settings.ocr_monitor_number != settings.icon_monitor_number {
        let ocr_image = ocr_monitor.capture_image().unwrap();
        let icon_image = icon_monitor.capture_image().unwrap();

        ocr_image
            .save(format!("monitor-ocr-{}.png", settings.ocr_monitor_number))
            .unwrap();

        icon_image
            .save(format!("monitor-icon-{}.png", settings.icon_monitor_number))
            .unwrap();
    } else {
        let image = icon_monitor.capture_image().unwrap();

        image
            .save(format!("monitor-ocr-{}.png", settings.ocr_monitor_number))
            .unwrap();

        image
            .save(format!("monitor-icon-{}.png", settings.icon_monitor_number))
            .unwrap();
    }

    // TODO: Read "Day" and "Night" using rusty-tesseract
    // Crate: https://github.com/thomasgruebl/rusty-tesseract

    // TODO: Use OpenCV to find Day / Night Shader icons
    // Python Example: https://stackoverflow.com/questions/7853628/how-do-i-find-an-image-contained-within-an-image
    // Crate: https://github.com/twistedfall/opencv-rust

    // TODO: Add working User Interface
    // Crate: https://v2.tauri.app/

    println!("Time Elapsed: {:?}", start.elapsed());
}
