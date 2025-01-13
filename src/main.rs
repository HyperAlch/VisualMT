mod ocr_state;
mod screen_cap;
mod settings;

use serde::{Deserialize, Serialize};
use std::env;

const SETTINGS_ERROR_SPACE: isize = 100;
const SCREEN_CAP_ERROR_SPACE: isize = 200;

fn main() {
    // TODO: Read "Day" and "Night" using rusty-tesseract
    // Crate: https://github.com/thomasgruebl/rusty-tesseract
    /*
       let output = rusty_tesseract::image_to_string(&ocr_image.into(), &my_args).unwrap();
       println!("The String output is: {:?}", output);
    */

    // TODO: Use OpenCV to find Day / Night Shader icons
    // Python Example: https://stackoverflow.com/questions/7853628/how-do-i-find-an-image-contained-within-an-image
    // Crate: https://github.com/twistedfall/opencv-rust

    // TODO: Add working User Interface
    // Crate: https://v2.tauri.app/

    println!("Time Elapsed: {:?}", start.elapsed());
}
