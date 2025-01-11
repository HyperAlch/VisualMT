mod settings;

use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use xcap::Monitor;

const SETTINGS_ERROR_SPACE: isize = 100;

fn main() {
    let settings = Settings {
        monitor_number: 0,
        x: 0,
        y: 0,
        width: 500,
        height: 300,
    };
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();
    let monitor = monitors.get(settings.monitor_number).unwrap();

    let image = monitor.capture_image().unwrap();

    image
        .save(format!("target/monitor-{}.png", settings.monitor_number))
        .unwrap();

    println!("Time Elapsed: {:?}", start.elapsed());
}
