mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use image::{png::PngEncoder, ImageEncoder, Rgb, RgbImage};
use js_sys::Uint8Array;

#[wasm_bindgen]
extern "C" {
    // fn alert(s: &str);
}

#[wasm_bindgen]
pub fn mandelbrot(width: u32, height: u32, max_iterations: usize) -> Uint8Array {
    let width_ratio = 3.5f64 / width as f64;
    let height_ratio = 2f64 / height as f64;
    let mut image = RgbImage::new(width as u32, height as u32);

    for py in 0..height {
        let y0 = (py as f64) * height_ratio - 1f64;
        for px in 0..width {
            let x0 = (px as f64) * width_ratio - 2.5f64;
            let mut x = 0f64;
            let mut y = 0f64;
            let mut iteration = 0;
            while x * x + y * y <= (1 << 16) as f64 && iteration < max_iterations {
                let xtemp = x * x + y * y + x0;
                y = 2f64 * x * y + y0;
                x = xtemp;
                iteration += 1;
            }
            // TODO: Wat do?
            // if iteration < max_iterations {
            //     let log_zn = f64::log2(x * x + y * y) / 2f64;
            //     let nu = f64::log2(log_zn / )
            // }
            let color = ((iteration as f64) / (max_iterations as f64) * 255f64) as u8;
            image.put_pixel(px, py, Rgb([color, color, 255]));
        }
    }

    let mut vec = vec![];
    let encoder = PngEncoder::new(&mut vec);
    let res = encoder.write_image(&image, width, height, image::ColorType::Rgb8);
    res.expect("Error incoding image");
    unsafe { Uint8Array::view(&vec) }
}
