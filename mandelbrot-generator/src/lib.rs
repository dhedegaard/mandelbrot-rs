mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use bmp::{self, Image, Pixel};
use js_sys::Uint8Array;

#[wasm_bindgen]
extern "C" {}

const WIDTH_HEIGHT_RATIO: f32 = 1.2;

#[wasm_bindgen]
pub fn mandelbrot(width: f32, height: f32, max_iterations: usize) -> Uint8Array {
    let ratio: f32 = (width / height).into();
    let relative_width: f32 = if ratio > WIDTH_HEIGHT_RATIO {
        width / ratio
    } else {
        width as f32
    };
    let relative_height: f32 = if ratio < WIDTH_HEIGHT_RATIO {
        height * ratio
    } else {
        height as f32
    };
    let width_ratio = 3.5f32 / relative_width;
    let height_ratio = 2f32 / relative_height;
    let mut bmp = Image::new(width as u32, height as u32);

    let max_iterations_f = max_iterations as f32;
    for py in 0..height as u32 {
        let y0 = (py as f32) * height_ratio - 1f32;
        for px in 0..width as u32 {
            let x0 = (px as f32) * width_ratio - 2.5f32;
            let mut x = 0f32;
            let mut y = 0f32;
            let mut iteration = 0f32;
            while x * x + y * y <= 65536f32 && iteration < max_iterations_f {
                let xtemp = x * x - y * y + x0;
                y = 2f32 * x * y + y0;
                x = xtemp;
                iteration += 1f32;
            }
            // If we exceeded the iterations, paint it white.
            if iteration >= max_iterations_f {
                continue;
            }
            let log_zn = (x * x + y * y).log10() / 2f32;
            let nu = f32::log10(log_zn / f32::log10(2f32)) / f32::log10(2f32);
            iteration = iteration + 1f32 - nu;
            let color1 = iteration / (max_iterations_f) * 255f32;
            let color2 = 1f32 + iteration / (max_iterations_f) * 255f32;
            let t = iteration % 1f32;
            let color = f32::round(t * color1 + (1f32 - t) * color2) as u32;
            bmp.set_pixel(
                px,
                py,
                Pixel {
                    r: color as u8,
                    g: color as u8,
                    b: color as u8,
                },
            );
        }
    }
    let mut vec = vec![];
    bmp.to_writer(&mut vec)
        .expect("Error writing BMP to bit array");
    unsafe { Uint8Array::view(&vec) }
}
