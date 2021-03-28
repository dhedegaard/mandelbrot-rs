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
pub fn mandelbrot(width: f32, height: f32, max_iterations: usize) -> Uint8Array {
    let ratio: f32 = (width / height).into();
    let relative_width: f32 = if ratio > 1.0 {
        width / ratio
    } else {
        width as f32
    };
    let relative_height: f32 = if ratio < 1.0 {
        height * ratio
    } else {
        height as f32
    };
    let width_ratio = 3.5f32 / relative_width as f32;
    let height_ratio = 2f32 / relative_height as f32;
    let mut image = RgbImage::new(width as u32, height as u32);

    for py in 0..height as u32 {
        let y0 = (py as f32) * height_ratio - 1f32;
        for px in 0..width as u32 {
            let x0 = (px as f32) * width_ratio - 2.5f32;
            let mut x = 0f32;
            let mut y = 0f32;
            let mut iteration = 0f32;
            while x * x + y * y <= (1 << 16) as f32 && iteration < max_iterations as f32 {
                let xtemp = x * x - y * y + x0;
                y = 2f32 * x * y + y0;
                x = xtemp;
                iteration += 1f32;
            }
            // If we exceeded the iterations, paint it white.
            if iteration >= max_iterations as f32 {
                image.put_pixel(px, py, Rgb([255, 255, 255]));
                continue;
            }
            let log_zn = (x * x + y * y).log10() / 2f32;
            let nu = f32::log10(log_zn / f32::log10(2f32)) / f32::log10(2f32);
            iteration = iteration + 1f32 - nu;
            let color1 = iteration / (max_iterations as f32) * 255f32;
            let color2 = 1f32 + iteration / (max_iterations as f32) * 255f32;
            let t = iteration % 1f32;
            let color = f32::round(t * color1 + (1f32 - t) * color2) as u8;
            image.put_pixel(px, py, Rgb([color, color, 255]));
        }
    }

    let mut vec = vec![];
    let encoder = PngEncoder::new(&mut vec);
    let res = encoder.write_image(&image, width as u32, height as u32, image::ColorType::Rgb8);
    res.expect("Error incoding image");
    unsafe { Uint8Array::view(&vec) }
}
