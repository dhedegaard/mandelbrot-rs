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
extern "C" {}

#[wasm_bindgen]
pub fn mandelbrot(width: f64, height: f64, max_iterations: usize) -> Uint8Array {
    let ratio: f64 = (width / height).into();
    let relative_width: f64 = if ratio > 1.0 {
        width / ratio
    } else {
        width as f64
    };
    let relative_height: f64 = if ratio < 1.0 {
        height * ratio
    } else {
        height as f64
    };
    // let relative_width = width;
    // let relative_height = height;
    let width_ratio = 3.5f64 / relative_width as f64;
    let height_ratio = 2f64 / relative_height as f64;
    let mut image = RgbImage::new(width as u32, height as u32);

    for py in 0..height as u32 {
        let y0 = (py as f64) * height_ratio - 1f64;
        for px in 0..width as u32 {
            let x0 = (px as f64) * width_ratio - 2.5f64;
            let mut x = 0f64;
            let mut y = 0f64;
            let mut iteration = 0f64;
            while x * x + y * y <= (1 << 16) as f64 && iteration < max_iterations as f64 {
                let xtemp = x * x - y * y + x0;
                y = 2f64 * x * y + y0;
                x = xtemp;
                iteration += 1f64;
            }
            // If we exceeded the iterations, paint it white.
            if iteration >= max_iterations as f64 {
                image.put_pixel(px, py, Rgb([255, 255, 255]));
                continue;
            }
            let log_zn = (x * x + y * y).log10() / 2f64;
            let nu = f64::log10(log_zn / f64::log10(2f64)) / f64::log10(2f64);
            iteration = iteration + 1f64 - nu;
            let color1 = iteration / (max_iterations as f64) * 255f64;
            let color2 = 1f64 + iteration / (max_iterations as f64) * 255f64;
            let t = iteration % 1f64;
            let color = f64::round(t * color1 + (1f64 - t) * color2) as u8;
            image.put_pixel(px, py, Rgb([color, color, 255]));
        }
    }

    let mut vec = vec![];
    let encoder = PngEncoder::new(&mut vec);
    let res = encoder.write_image(&image, width as u32, height as u32, image::ColorType::Rgb8);
    res.expect("Error incoding image");
    unsafe { Uint8Array::view(&vec) }
}
