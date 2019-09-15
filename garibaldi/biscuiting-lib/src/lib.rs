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

fn format_histogram<P, F, PS>(image: &imageproc::definitions::Image<P>, key_format_fn: F) -> String
where
    F: Fn(&P) -> String,
    PS: image::Primitive + 'static,
    P: image::Pixel<Subpixel = PS> + 'static,
{
    use std::collections::HashMap;

    let pixel_counts: HashMap<String, usize> =
        image.pixels().fold(HashMap::new(), |mut counts, pixel| {
            let formatted_pixel = key_format_fn(pixel);
            counts.insert(
                formatted_pixel.clone(),
                match counts.get(&formatted_pixel) {
                    Some(count) => count + 1,
                    None => 1,
                },
            );
            counts
        });

    let mut keys: Vec<String> = pixel_counts.keys().map(|k| k.clone()).collect();
    keys.sort();
    let summary: Vec<String> = keys
        .iter()
        .map(|k| format!("[{}]: {}", k, pixel_counts[k]))
        .collect();

    summary.join(", ")
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
        use image::Luma;
        use imageproc::map::map_colors;
        use imageproc::region_labelling::{connected_components, Connectivity};
        use web_sys::console;

        console::log_1(
            &format!(
                "{}, {}, {}, {}",
                input.0[0], input.0[1], input.0[2], input.0[3]
            )
            .into(),
        );
        console::time_with_label("from raw input");
        match RgbaImage::from_raw(self.width, self.height, input.0) {
            Some(image) => {
                console::time_end_with_label("from raw input");
                console::log_1(
                    &format_histogram(&image, |pixel: &Rgba<u8>| {
                        format!("{}, {}, {}, {}", pixel[0], pixel[1], pixel[2], pixel[3])
                    })
                    .into(),
                );

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

                console::log_1(
                    &format_histogram(&binarised_image, |pixel: &Luma<u8>| format!("{}", pixel[0]))
                        .into(),
                );

                let labelled_image =
                    connected_components(&binarised_image, Connectivity::Four, background_color);

                console::log_1(
                    &format_histogram(&labelled_image, |pixel: &Luma<u32>| format!("{}", pixel[0]))
                        .into(),
                );

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

    pub fn find_biscuits_simple(&mut self, input: Clamped<Vec<u8>>) -> Result<String, JsValue> {
        use image::{GrayImage, Luma};
        use imageproc::map::map_colors;
        use imageproc::region_labelling::{connected_components, Connectivity};
        use web_sys::console;

        let len = input.0.len();
        console::log_1(
            &format!(
                "raw input [{}, {}, {}, {}], [{}, {}, {}, {}]",
                input.0[0],
                input.0[1],
                input.0[2],
                input.0[3],
                input.0[len - 4],
                input.0[len - 3],
                input.0[len - 2],
                input.0[len - 1]
            )
            .into(),
        );

        console::time_with_label("from raw input");
        match RgbaImage::from_raw(self.width, self.height, input.0) {
            Some(image) => {
                console::time_end_with_label("from raw input");

                let top_left = image.get_pixel(0, 0);
                let bottom_right = image.get_pixel(self.width - 1, self.height - 1);
                console::log_1(
                    &format!(
                        "rgba [{}, {}, {}, {}], [{}, {}, {}, {}]",
                        top_left[0],
                        top_left[1],
                        top_left[2],
                        top_left[3],
                        bottom_right[0],
                        bottom_right[1],
                        bottom_right[2],
                        bottom_right[3]
                    )
                    .into(),
                );

                console::time_with_label("process image");
                let gray_image: GrayImage = map_colors(&image, |p| {
                    let avg = ((p[0] as f32) + (p[1] as f32) + (p[2] as f32)) / 3.0;
                    let alpha = p[3] as f32 / std::u8::MAX as f32;
                    let gray = (alpha * avg).floor() as u8;
                    let inverted_gray = 255 - gray;
                    Luma([inverted_gray])
                });

                let background_color = Luma([0u8; 1]);
                let labelled_image =
                    connected_components(&gray_image, Connectivity::Four, background_color);
                let num_labels = (labelled_image.pixels().map(|p| p[0]).max().unwrap()) as usize;
                let color_map = self.random_color_map(num_labels + 1);
                let processed_gray_image =
                    map_colors(&labelled_image, |p| color_map[p[0] as usize]);

                console::time_end_with_label("process image");

                console::time_with_label("to raw output");
                // let remapped_to_rgba_image: RgbaImage = map_colors(&gray_image, |p| {
                let remapped_to_rgba_image: RgbaImage = map_colors(&processed_gray_image, |p| {
                    let gray = p[0];
                    Rgba([gray, gray, gray, 255])
                });
                self.output = Some(remapped_to_rgba_image.to_vec());
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
