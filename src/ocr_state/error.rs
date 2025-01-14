use crate::OCR_ERROR_SPACE;
use std::fmt;
use std::fmt::Formatter;

pub(crate) struct OcrError {
    code: OcrErrorCode,
    message: Option<String>,
}

#[derive(Debug)]
enum OcrErrorCode {
    ImageToTextFailed = OCR_ERROR_SPACE,
}

impl fmt::Display for OcrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            OcrErrorCode::ImageToTextFailed => &format!("Internal Error: {:?}", self.code),
        };

        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for OcrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ocr Error {{ code: {:?}, message: {} }}",
            self.code,
            self.message.as_ref().unwrap_or(&String::new())
        )
    }
}

impl OcrError {
    fn new<T: ToString>(code: OcrErrorCode, message: Option<T>) -> Self {
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
