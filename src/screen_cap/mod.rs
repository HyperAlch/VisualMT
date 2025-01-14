mod error;

use crate::screen_cap::error::ScreenCapError;
use std::fmt::Display;
use xcap::image::imageops::crop;
use xcap::image::RgbaImage;

fn capture_area_from_image<T: std::fmt::Display>(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<RgbaImage, ScreenCapError> {
    if x + width > image.width() || y + height > image.height() {
        return Err(ScreenCapError::new::<T>(
            error::ScreenCapErrorCode::SubImageOutOfBounds,
            None,
        ));
    }

    let cropped_area = crop(image, x, y, width, height).to_image();
    Ok(cropped_area)
}
#[cfg(test)]

mod tests {
    use crate::settings::Settings;
    use std::time::Instant;
    use xcap::Monitor;

    #[test]
    fn test_screen_cap() {
        // Load settings
        let mut settings = Settings::default();
        settings.load().unwrap();

        let result = std::fs::create_dir_all("./process-images/");
        if result.is_err() {
            println!("Error creating directory: {}", result.unwrap_err());
        }

        // Start XCap and save screenshots
        let start = Instant::now();
        let monitors = Monitor::all().unwrap();
        let ocr_monitor = monitors
            .get(settings.ocr_monitor_number)
            .expect("Monitor not found");
        let icon_monitor = monitors
            .get(settings.icon_monitor_number)
            .expect("Monitor not found");

        let ocr_image = ocr_monitor.capture_image().unwrap();
        let icon_image = icon_monitor.capture_image().unwrap();

        if settings.ocr_monitor_number != settings.icon_monitor_number {
            ocr_image
                .save(format!(
                    "./process-images/monitor-ocr-{}.png",
                    settings.ocr_monitor_number
                ))
                .unwrap();

            icon_image
                .save(format!(
                    "./process-images/monitor-icon-{}.png",
                    settings.icon_monitor_number
                ))
                .unwrap();
        } else {
            let image = icon_monitor.capture_image().unwrap();

            image
                .save(format!(
                    "./process-images/monitor-ocr-{}.png",
                    settings.ocr_monitor_number
                ))
                .unwrap();

            image
                .save(format!(
                    "./process-images/monitor-icon-{}.png",
                    settings.icon_monitor_number
                ))
                .unwrap();
        }

        println!("Time Elapsed: {:?}", start.elapsed());
    }
}
