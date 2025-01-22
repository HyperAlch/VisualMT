mod error;

use crate::settings::error::SettingsError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::sync::{Mutex, OnceLock};
pub(crate) static SETTINGS: OnceLock<Mutex<Settings>> = OnceLock::new();

// Function to initialize the global Settings
pub fn init_settings() {
    // This will initialize the SETTINGS global variable once
    SETTINGS
        .set(Mutex::new(Settings::default()))
        .unwrap_or_else(|_| {
            panic!("Settings were already initialized");
        });
}

// Function to get a thread-safe reference to the global Settings
pub fn global_settings() -> &'static Mutex<Settings> {
    SETTINGS
        .get()
        .expect("Settings should be initialized first")
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub(crate) ocr_x: u32,
    pub(crate) ocr_y: u32,
    pub(crate) ocr_width: u32,
    pub(crate) ocr_height: u32,
    pub(crate) icon_x: u32,
    pub(crate) icon_y: u32,
    pub(crate) icon_width: u32,
    pub(crate) icon_height: u32,
}

impl Settings {
    pub(crate) fn save(&self) -> Result<(), SettingsError> {
        // Serialize data to json
        let json_data = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => {
                return Err(SettingsError::new(
                    error::SettingsErrorCode::SerializeError,
                    Some(e),
                ))
            }
        };

        // Create file and return handle
        let mut file = match File::create("settings.json") {
            Ok(f) => f,
            Err(_) => {
                return Err(SettingsError::new(
                    error::SettingsErrorCode::IoError,
                    Some("Failed to create settings file"),
                ))
            }
        };

        // Rewrite json data using file handle
        match file.write_all(json_data.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(SettingsError::new(
                error::SettingsErrorCode::IoError,
                Some("Failed to write to settings file"),
            )),
        }
    }
    pub(crate) fn load(&mut self) -> Result<(), SettingsError> {
        // Deserialize the JSON data into the Settings struct
        let file = match File::open("settings.json") {
            Ok(f) => f,
            Err(_) => {
                return Err(SettingsError::new(
                    error::SettingsErrorCode::IoError,
                    Some("Failed to open settings file"),
                ))
            }
        };

        // Deserialize the JSON data into the Settings struct
        let json_data = match serde_json::from_reader(file) {
            Ok(settings) => settings,
            Err(e) => {
                return Err(SettingsError::new(
                    error::SettingsErrorCode::DeserializeError,
                    Some(e.to_string()),
                ))
            }
        };

        *self = json_data;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::{global_settings, init_settings, Settings};

    #[test]
    fn test_save_and_load_settings() {
        test_save_settings();
        test_load_settings();
        test_save_and_load_static_settings();
    }
    fn test_save_and_load_static_settings() {
        init_settings(); // Initialize SETTINGS at the beginning

        {
            // Test saving settings
            let settings = global_settings();
            let mut writable_settings = settings
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            writable_settings.ocr_x = 0;
            writable_settings.ocr_y = 0;
            writable_settings.ocr_width = 5000;
            writable_settings.ocr_height = 300;
            writable_settings.icon_x = 100;
            writable_settings.icon_y = 100;
            writable_settings.icon_width = 50;
            writable_settings.icon_height = 50;

            if let Err(e) = writable_settings.save() {
                panic!("{:?}", e);
            }
        }

        {
            // Test loading settings
            let settings = global_settings();
            let mut writable_settings = settings
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            if let Err(e) = writable_settings.load() {
                panic!("{:?}", e);
            }

            println!("{:?}", *writable_settings); // Log loaded settings
        }
    }

    fn test_save_settings() {
        let settings = Settings {
            ocr_x: 0,
            ocr_y: 0,
            ocr_width: 600,
            ocr_height: 300,

            icon_x: 100,
            icon_y: 100,
            icon_width: 50,
            icon_height: 50,
        };

        match settings.save() {
            Ok(_) => {}
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    fn test_load_settings() {
        let mut settings = Settings::default();

        match settings.load() {
            Ok(_) => {
                println!("{:?}", settings);
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
}
