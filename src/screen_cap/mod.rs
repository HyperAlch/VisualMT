use crate::SCREEN_CAP_ERROR_SPACE;
use std::fmt;
use std::fmt::{Display, Formatter};
use xcap::image::imageops::crop;
use xcap::image::RgbaImage;

pub(crate) struct ScreenCapError {
    code: ScreenCapErrorCode,
    message: Option<String>,
}

#[derive(Debug)]
enum ScreenCapErrorCode {
    MonitorNotFound = SCREEN_CAP_ERROR_SPACE,
    SubImageOutOfBounds,
}

impl fmt::Display for ScreenCapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            ScreenCapErrorCode::MonitorNotFound => &format!(
                "Cannot find monitor: {}",
                self.message.as_ref().unwrap_or(&String::new())
            ),
            ScreenCapErrorCode::SubImageOutOfBounds => &format!("Internal Error: {:?}", self.code),
        };

        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for ScreenCapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ScreenCap Error {{ code: {:?}, message: {} }}",
            self.code,
            self.message.as_ref().unwrap_or(&String::new())
        )
    }
}

impl ScreenCapError {
    fn new<T: ToString>(code: ScreenCapErrorCode, message: Option<T>) -> ScreenCapError {
        let new_message: Option<String>;
        if let Some(message) = message {
            new_message = Some(message.to_string());
        } else {
            new_message = None
        }

        Self {
            code,
            message: new_message,
        }
    }
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

fn capture_area_from_image<T: std::fmt::Display>(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<RgbaImage, ScreenCapError> {
    if x + width > image.width() || y + height > image.height() {
        return Err(ScreenCapError::new::<T>(
            ScreenCapErrorCode::SubImageOutOfBounds,
            None,
        ));
    }

    let cropped_area = crop(image, x, y, width, height).to_image();
    Ok(cropped_area)
}
