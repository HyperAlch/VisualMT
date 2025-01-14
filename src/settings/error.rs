use crate::SETTINGS_ERROR_SPACE;
use std::fmt;
use std::io::Write;
pub(crate) struct SettingsError {
    code: SettingsErrorCode,
    message: Option<String>,
}
#[derive(Debug)]
pub(crate) enum SettingsErrorCode {
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
    pub(crate) fn new<T: ToString>(code: SettingsErrorCode, message: Option<T>) -> Self {
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
