mod error;

use crate::normalize_filename;
use crate::screen_cap::error::ScreenCapError;
use std::collections::HashMap;
use std::fmt::Display;
use std::string::String;
use xcap::image::imageops::crop;
use xcap::image::RgbaImage;
use xcap::Window;

#[derive(Debug)]
struct ScreenCap {
    target_window: Window,
}

impl ScreenCap {
    pub(crate) fn new(target_window: Window) -> Self {
        Self { target_window }
    }

    pub(crate) fn capture_area(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<RgbaImage, ScreenCapError> {
        let mut image = match self.target_window.capture_image() {
            Ok(i) => i,
            Err(e) => {
                return Err(ScreenCapError::new::<String>(
                    error::ScreenCapErrorCode::SubImageOutOfBounds,
                    Some(e.to_string()),
                ));
            }
        };

        if x + width > image.width() || y + height > image.height() {
            let error_message = format!(
                "\nImage Width / Height: {}/{}\nX, Y: {},{}\nCapture Area Width / Height: {}/{}\n",
                image.width(),
                image.height(),
                x,
                y,
                width,
                height,
            );

            let error = Err(ScreenCapError::new::<String>(
                error::ScreenCapErrorCode::SubImageOutOfBounds,
                Some(error_message),
            ));
            return error;
        }

        let cropped_area = crop(&mut image, x, y, width, height).to_image();
        Ok(cropped_area)
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

    pub(crate) fn window_has_resized(&self, window_name: &str) -> bool {
        todo!()
    }
}

#[cfg(test)]

mod tests {
    use crate::screen_cap::{ScreenCap, WindowList};
    use crate::settings;
    use crate::settings::{global_settings, init_settings};

    #[test]
    fn capture_targets() {
        init_settings();

        // Get global settings, load into a local version
        let mut local_settings = settings::Settings::default();
        {
            let settings = global_settings();
            let mut writable_settings = settings
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            if let Err(e) = writable_settings.load() {
                panic!("{:?}", e);
            }

            local_settings.ocr_x = writable_settings.ocr_x;
            local_settings.ocr_y = writable_settings.ocr_y;
            local_settings.ocr_width = writable_settings.ocr_width;
            local_settings.ocr_height = writable_settings.ocr_height;

            local_settings.icon_x = writable_settings.icon_x;
            local_settings.icon_y = writable_settings.icon_y;
            local_settings.icon_width = writable_settings.icon_width;
            local_settings.icon_height = writable_settings.icon_height;
        }

        // Get target window
        let all_windows = WindowList::new();
        let window = all_windows.filter_by_title("GW2");
        let window = window.get_vec();
        let window = ScreenCap::new(window.get(0).unwrap().1.clone());

        // Capture and save OCR image
        let captured_ocr_image = window
            .capture_area(
                local_settings.ocr_x,
                local_settings.ocr_y,
                local_settings.ocr_width,
                local_settings.ocr_height,
            )
            .unwrap()
            .save("./process-images/ocr_target.png")
            .unwrap();

        // Capture and save icon image
        let captured_icon_image = window
            .capture_area(
                local_settings.icon_x,
                local_settings.icon_y,
                local_settings.icon_width,
                local_settings.icon_height,
            )
            .unwrap()
            .save("./process-images/icon_target.png")
            .unwrap();
    }
}
