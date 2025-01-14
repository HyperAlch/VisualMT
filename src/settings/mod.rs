use crate::SETTINGS_ERROR_SPACE;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Write;

pub(crate) struct SettingsError {
    code: SettingsErrorCode,
    message: Option<String>,
}
#[derive(Debug)]
enum SettingsErrorCode {
    IoError = SETTINGS_ERROR_SPACE,
    SerializeError,
    DeserializeError,
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            SettingsErrorCode::IoError => &format!(
                "Issue With Settings File: {}",
                self.message.as_ref().unwrap_or(&String::new())
            ),
            SettingsErrorCode::SerializeError | SettingsErrorCode::DeserializeError => {
                &format!("Internal Error: {:?}", self.code)
            }
        };

        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingsError {{ code: {:?}, message: {} }}",
            self.code,
            self.message.as_ref().unwrap_or(&String::new())
        )
    }
}

impl SettingsError {
    fn new<T: ToString>(code: SettingsErrorCode, message: Option<T>) -> SettingsError {
        let new_message: Option<String>;
        if let Some(message) = message {
            new_message = Some(message.to_string());
        } else {
            new_message = None
        }

        SettingsError {
            code,
            message: new_message,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub(crate) ocr_monitor_number: usize,
    pub(crate) ocr_x: u16,
    pub(crate) ocr_y: u16,
    pub(crate) ocr_width: u16,
    pub(crate) ocr_height: u16,
    pub(crate) icon_monitor_number: usize,
    pub(crate) icon_x: u16,
    pub(crate) icon_y: u16,
    pub(crate) icon_width: u16,
    pub(crate) icon_height: u16,
}

impl Settings {
    pub(crate) fn save(&self) -> Result<(), SettingsError> {
        // Serialize data to json
        let json_data = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => {
                return Err(SettingsError::new(
                    SettingsErrorCode::SerializeError,
                    Some(e),
                ))
            }
        };

        // Create file and return handle
        let mut file = match File::create("settings.json") {
            Ok(f) => f,
            Err(_) => {
                return Err(SettingsError::new(
                    SettingsErrorCode::IoError,
                    Some("Failed to create settings file"),
                ))
            }
        };

        // Rewrite json data using file handle
        match file.write_all(json_data.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(SettingsError::new(
                SettingsErrorCode::IoError,
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
                    SettingsErrorCode::IoError,
                    Some("Failed to open settings file"),
                ))
            }
        };

        // Deserialize the JSON data into the Settings struct
        let json_data = match serde_json::from_reader(file) {
            Ok(settings) => settings,
            Err(e) => {
                return Err(SettingsError::new(
                    SettingsErrorCode::DeserializeError,
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
    use crate::settings::Settings;

    #[test]
    fn test_save_settings() {
        let settings = Settings {
            ocr_monitor_number: 0,
            ocr_x: 0,
            ocr_y: 0,
            ocr_width: 500,
            ocr_height: 300,

            icon_monitor_number: 0,
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

    #[test]
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
