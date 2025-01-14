use crate::SCREEN_CAP_ERROR_SPACE;
use std::fmt;
use std::fmt::{Display, Formatter};

pub(crate) struct ScreenCapError {
    code: ScreenCapErrorCode,
    message: Option<String>,
}

#[derive(Debug)]
pub(crate) enum ScreenCapErrorCode {
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
    pub(crate) fn new<T: ToString>(code: ScreenCapErrorCode, message: Option<T>) -> Self {
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
