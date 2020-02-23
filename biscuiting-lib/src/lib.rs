extern crate console_error_panic_hook;
extern crate image;
#[macro_use]
extern crate imageproc;
extern crate js_sys;
extern crate wasm_bindgen;
#[cfg(feature = "wee_alloc")]
extern crate wee_alloc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod region_labelling;

#[wasm_bindgen]
pub struct BiscuitFinder {
    border_indexes: Option<Vec<usize>>,
    border_points: Option<Vec<f32>>,
}

use image::{Rgba, RgbaImage};

impl Default for BiscuitFinder {
    fn default() -> Self {
        BiscuitFinder {
            border_indexes: None,
            border_points: None,
        }
    }
}

#[wasm_bindgen]
impl BiscuitFinder {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        use web_sys::console;
        console::log_1(&format!("BiscuitFinder version {}", VERSION).into());
        Self::default()
    }

    pub fn find_biscuits(
        &mut self,
        width: u32,
        height: u32,
        input: Clamped<Vec<u8>>,
        x_offset: f32,
        y_offset: f32,
        scale_down: f32,
    ) -> Result<String, JsValue> {
        use image::{GrayImage, Luma};
        use imageproc::definitions::Image;
        use imageproc::map::map_colors;
        use imageproc::region_labelling::{connected_components, Connectivity};
        let input_background_color = Rgba([255u8; 4]);

        match RgbaImage::from_raw(width, height, input.0) {
            Some(image) => {
                let foreground_color = Luma([255u8; 1]);
                let background_color = Luma([0u8; 1]);

                let gray_image: GrayImage = map_colors(&image, |p| {
                    if p == input_background_color {
                        background_color
                    } else {
                        foreground_color
                    }
                });

                let labelled_image: Image<Luma<u32>> =
                    connected_components(&gray_image, Connectivity::Four, background_color);
                let contours = region_labelling::find_contours(Luma([0u32; 1]), &labelled_image);
                let mut border_indexes = Vec::new();
                let mut border_points = Vec::new();
                let mut start_index: usize = 0;
                for contour in contours {
                    let indexes_used = contour.len() * 2;
                    border_indexes.push(start_index + indexes_used);
                    start_index += indexes_used;
                    for point in contour {
                        let x = x_offset + (point.x as f32 / scale_down);
                        let y = y_offset + (point.y as f32 / scale_down);
                        border_points.push(x);
                        border_points.push(y);
                    }
                }
                self.border_indexes = Some(border_indexes);
                self.border_points = Some(border_points);
                Ok("processed image".into())
            }
            None => Err("couldn't read from raw".into()),
        }
    }

    pub fn border_indexes_ptr(&self) -> *const usize {
        match &self.border_indexes {
            Some(vec) => vec.as_ptr(),
            None => panic!("no border indexes"),
        }
    }

    pub fn num_borders(&self) -> usize {
        match &self.border_indexes {
            Some(vec) => vec.len(),
            None => panic!("no borders"),
        }
    }

    pub fn border_points_ptr(&self) -> *const f32 {
        match &self.border_points {
            Some(vec) => vec.as_ptr(),
            None => panic!("no border points"),
        }
    }

    pub fn num_border_points(&self) -> usize {
        match &self.border_points {
            Some(vec) => vec.len() / 2,
            None => panic!("no border points"),
        }
    }
}

impl BiscuitFinder {
    pub fn border_indexes(&self) -> Result<Vec<usize>, String> {
        match &self.border_indexes {
            Some(vec) => Ok(vec.clone()),
            None => panic!("no border points"),
        }
    }

    pub fn border_points(&self) -> Result<Vec<f32>, String> {
        match &self.border_points {
            Some(vec) => Ok(vec.clone()),
            None => panic!("no border points"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate wasm_bindgen_test;
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_when_only_background_pixels_provided() {
        let mut biscuit_finder = BiscuitFinder::new();

        let image = rgba_image!(
            [255, 255, 255, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [255, 255, 255, 255]);

        let input = Clamped(image.to_vec());
        let result = biscuit_finder.find_biscuits(2, 2, input, 0.0, 0.0, 1.0);

        assert_eq!(Ok("processed image".into()), result);

        assert_eq!(0, biscuit_finder.num_borders());
        let border_points = biscuit_finder.border_points();
        assert_eq!(Ok(vec![]), border_points);
        let border_indexes = biscuit_finder.border_indexes();
        assert_eq!(Ok(vec![]), border_indexes);
    }

    #[wasm_bindgen_test]
    fn test_with_single_pixel_biscuit_in_top_left_corner() {
        let mut biscuit_finder = BiscuitFinder::new();

        let image = rgba_image!(
            [0,     0,   0, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [255, 255, 255, 255]);

        let input = Clamped(image.to_vec());
        let result = biscuit_finder.find_biscuits(2, 2, input, 0.0, 0.0, 1.0);

        assert_eq!(Ok("processed image".into()), result);

        assert_eq!(1, biscuit_finder.num_borders());
        let border_points = biscuit_finder.border_points();
        assert_eq!(Ok(vec![0.0, 0.0]), border_points);
        let border_indexes = biscuit_finder.border_indexes();
        assert_eq!(Ok(vec![2]), border_indexes);
    }

    #[wasm_bindgen_test]
    fn test_with_big_biscuit_in_middle() {
        let mut biscuit_finder = BiscuitFinder::new();

        let image = rgba_image!(
            [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [0,     0,   0, 255], [0,     0,   0, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [0,     0,   0, 255], [0,     0,   0, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255]);

        let input = Clamped(image.to_vec());
        let result = biscuit_finder.find_biscuits(4, 4, input, 0.0, 0.0, 1.0);

        assert_eq!(Ok("processed image".into()), result);

        assert_eq!(1, biscuit_finder.num_borders());
        let border_points = biscuit_finder.border_points();
        assert_eq!(
            Ok(vec![1.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            border_points
        );
        let border_indexes = biscuit_finder.border_indexes();
        assert_eq!(Ok(vec![8]), border_indexes);
    }

    #[wasm_bindgen_test]
    fn test_with_multiple_single_pixel_biscuits() {
        let mut biscuit_finder = BiscuitFinder::new();

        let image = rgba_image!(
            [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [0,     0,   0, 255], [255, 255, 255, 255], [0,     0,   0, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [0,     0,   0, 255], [255, 255, 255, 255], [0,     0,   0, 255], [255, 255, 255, 255];
            [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255]);

        let input = Clamped(image.to_vec());
        let result = biscuit_finder.find_biscuits(5, 5, input, 0.0, 0.0, 1.0);

        assert_eq!(Ok("processed image".into()), result);

        assert_eq!(4, biscuit_finder.num_borders());
        let border_points = biscuit_finder.border_points();
        assert_eq!(
            Ok(vec![
                1.0, 1.0, //
                3.0, 1.0, //
                1.0, 3.0, //
                3.0, 3.0, //
            ]),
            border_points
        );
        let border_indexes = biscuit_finder.border_indexes();
        assert_eq!(Ok(vec![2, 4, 6, 8]), border_indexes);
    }

    #[wasm_bindgen_test]
    fn test_with_multiple_multi_pixel_biscuits() {
        let mut biscuit_finder = BiscuitFinder::new();

        let image = rgba_image!(
            [0,     0,   0, 255], [0,     0,   0, 255], [255, 255, 255, 255], [0,     0,   0, 255], [0,     0,   0, 255];
            [0,     0,   0, 255], [0,     0,   0, 255], [255, 255, 255, 255], [0,     0,   0, 255], [0,     0,   0, 255];
            [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255], [255, 255, 255, 255];
            [0,     0,   0, 255], [0,     0,   0, 255], [255, 255, 255, 255], [0,     0,   0, 255], [0,     0,   0, 255];
            [0,     0,   0, 255], [0,     0,   0, 255], [255, 255, 255, 255], [0,     0,   0, 255], [0,     0,   0, 255]);

        let input = Clamped(image.to_vec());
        let result = biscuit_finder.find_biscuits(5, 5, input, 0.0, 0.0, 1.0);

        assert_eq!(Ok("processed image".into()), result);

        assert_eq!(4, biscuit_finder.num_borders());
        let border_points = biscuit_finder.border_points();
        assert_eq!(
            Ok(vec![
                0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, //
                3.0, 0.0, 4.0, 0.0, 4.0, 1.0, 3.0, 1.0, //
                0.0, 3.0, 1.0, 3.0, 1.0, 4.0, 0.0, 4.0, //
                3.0, 3.0, 4.0, 3.0, 4.0, 4.0, 3.0, 4.0, //
            ]),
            border_points
        );
        let border_indexes = biscuit_finder.border_indexes();
        assert_eq!(Ok(vec![8, 16, 24, 32]), border_indexes);
    }
}
