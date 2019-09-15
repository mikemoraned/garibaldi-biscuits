extern crate base64;
extern crate console_error_panic_hook;
extern crate image;
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub struct BiscuitFinder {
    width: u32,
    height: u32,
    output: Option<Vec<u8>>,
}

use image::{Rgba, RgbaImage};

fn gen_range(min: u8, max: u8) -> u8 {
    let random = js_sys::Math::random();
    return ((random * ((max - min + 1) as f64)).floor() as u8) + min;
}

#[wasm_bindgen]
impl BiscuitFinder {
    pub fn new(width: u32, height: u32) -> BiscuitFinder {
        console_error_panic_hook::set_once();
        BiscuitFinder {
            width,
            height,
            output: None,
        }
    }

    fn random_color_map(&self, num_labels: usize) -> Vec<Rgba<u8>> {
        use web_sys::console;

        let mut color_map = vec![Rgba([0u8; 4]); num_labels];
        color_map[0] = Rgba([0u8; 4]);
        for label in 1..num_labels {
            color_map[label] = Rgba([gen_range(1, 255), gen_range(1, 255), gen_range(1, 255), 0u8]);
        }
        console::log_1(&format!("color map: {:?}", color_map).into());

        color_map
    }

    pub fn find_biscuits(&mut self, input: Clamped<Vec<u8>>) -> Result<String, JsValue> {
        // use imageproc::noise::salt_and_pepper_noise;
        use image::Luma;
        use imageproc::map::map_colors;
        use imageproc::region_labelling::{connected_components, Connectivity};
        use web_sys::console;

        console::time_with_label("from raw input");
        match RgbaImage::from_raw(self.width, self.height, input.0) {
            Some(image) => {
                console::time_end_with_label("from raw input");

                console::time_with_label("process image");
                let foreground_color = Luma([255u8; 1]);
                let background_color = Luma([0u8; 1]);
                console::time_with_label("find connected components");
                let binarised_image = map_colors(&image, |p| {
                    if p[0] > 0 {
                        return background_color;
                    } else {
                        return foreground_color;
                    }
                });
                let labelled_image =
                    connected_components(&binarised_image, Connectivity::Four, background_color);
                console::time_end_with_label("find connected components");
                console::time_with_label("build color map");
                let num_labels =
                    (labelled_image.pixels().map(|p| p[0]).max().unwrap() + 1) as usize;
                let color_map = self.random_color_map(num_labels);
                console::time_end_with_label("build color map");
                console::time_with_label("apply color map");
                let processed_image = map_colors(&labelled_image, |p| color_map[p[0] as usize]);
                console::time_end_with_label("apply color map");
                console::time_end_with_label("process image");

                console::time_with_label("to raw output");
                self.output = Some(processed_image.to_vec());
                console::time_end_with_label("to raw output");

                return Ok("processed image".into());
            }
            None => {
                return Err("couldn't read from raw".into());
            }
        }
    }

    pub fn output(&self) -> *const u8 {
        match &self.output {
            Some(buffer) => buffer.as_ptr(),
            None => panic!("no output"),
        }
    }
}
