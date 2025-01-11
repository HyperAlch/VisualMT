use crate::SETTINGS_ERROR_SPACE;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Write;

struct SettingsError {
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
            SettingsErrorCode::SerializeError => &format!(
                "Internal Error: {}",
                self.message.as_ref().unwrap_or(&String::new())
            ),
            _ => "Unknown Settings Error",
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub(crate) monitor_number: usize,
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl Settings {
    fn save(&self) -> Result<(), SettingsError> {
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
}

#[cfg(test)]
mod tests {
    use crate::settings::Settings;

    #[test]
    fn test_save_settings() {
        let settings = Settings {
            monitor_number: 0,
            x: 0,
            y: 0,
            width: 500,
            height: 300,
        };

        match settings.save() {
            Ok(_) => {}
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
}
