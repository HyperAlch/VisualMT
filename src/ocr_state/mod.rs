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
