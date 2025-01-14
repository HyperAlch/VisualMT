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

#[cfg(test)]
mod tests {
    use rusty_tesseract::image::ImageReader;
    use rusty_tesseract::{Args, Image};
    use std::collections::HashMap;

    #[test]
    fn detect_day_night_test() {
        let mut my_args = Args {
            //model language (tesseract default = 'eng')
            //available languages can be found by running 'rusty_tesseract::get_tesseract_langs()'
            lang: "eng".into(),

            //map of config variables
            //this example shows a whitelist for the normal alphabet. Multiple arguments are allowed.
            //available arguments can be found by running 'rusty_tesseract::get_tesseract_config_parameters()'
            config_variables: HashMap::from([(
                "tessedit_char_whitelist".into(),
                "uskawnightyDN".into(),
            )]),
            dpi: Some(150), // specify DPI for input image
            psm: Some(6), // define page segmentation mode 6 (i.e. "Assume a single uniform block of text")
            oem: Some(3), // define optical character recognition mode 3 (i.e. "Default, based on what is available")
        };

        let dynamic_image = ImageReader::open("./test-images/night.png")
            .unwrap()
            .decode()
            .unwrap();
        let img = Image::from_dynamic_image(&dynamic_image).unwrap();
        let output = rusty_tesseract::image_to_string(&img, &my_args).unwrap();
        assert!(output.contains("Night"), "Output: {}", output);

        let dynamic_image = ImageReader::open("./test-images/day.png")
            .unwrap()
            .decode()
            .unwrap();
        let img = Image::from_dynamic_image(&dynamic_image).unwrap();
        let output = rusty_tesseract::image_to_string(&img, &my_args).unwrap();
        assert!(output.contains("Day"), "Output: {}", output);
    }
}
