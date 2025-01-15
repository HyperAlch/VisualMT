mod error;

use crate::normalize_filename;
use crate::screen_cap::error::ScreenCapError;
use std::collections::HashMap;
use std::fmt::Display;
use xcap::image::imageops::crop;
use xcap::image::RgbaImage;
use xcap::Window;

fn capture_area_from_image<T: Display>(
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

#[derive(Debug)]
struct ScreenCap {
    target_window: Window,
}

impl ScreenCap {
    pub(crate) fn new(target_window: Window) -> Self {
        Self { target_window }
    }
}

#[derive(Debug)]
struct WindowList(HashMap<String, Window>);
impl<'a> Into<Vec<(&'a String, &'a Window)>> for &'a WindowList {
    fn into(self) -> Vec<(&'a String, &'a Window)> {
        self.0.iter().collect()
    }
}
impl WindowList {
    pub(crate) fn new() -> Self {
        Self(Self::list_all_windows())
    }

    pub(crate) fn filter_by_title(self, title: &str) -> Self {
        let filtered: HashMap<String, Window> = self
            .0
            .iter()
            .filter(|(key, _)| key.contains(title))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        let filtered = Self(filtered);
        filtered
    }

    pub(crate) fn get_vec(&self) -> Vec<(&String, &Window)> {
        self.into()
    }

    fn list_all_windows() -> HashMap<String, Window> {
        let windows = Window::all().unwrap();
        let mut i = 0;

        let mut window_map: HashMap<String, Window> = HashMap::new();
        let mut title = String::new();

        for window in windows {
            if window.is_minimized() {
                continue;
            }

            title = window.title().to_string();
            title = format!(
                "./process-images/window-{}-_-{}.png",
                i,
                normalize_filename(window.title())
            );

            window_map.insert(title, window);

            i += 1;
        }

        window_map
    }
}

#[cfg(test)]

mod tests {
    use crate::screen_cap::{ScreenCap, WindowList};
    use crate::settings::Settings;
    use xcap::Monitor;

    #[test]
    fn capture_window() {
        let all_windows = WindowList::new();
        // println!("{:#?}", all_windows);

        let window = all_windows.filter_by_title("GW2");
        let window = window.get_vec();

        let window = ScreenCap::new(window[0].1.clone());
        println!("{:#?}", window);
    }
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
    }
}
