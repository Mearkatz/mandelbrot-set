#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::cast_precision_loss,
    clippy::cast_lossless
)]

use beetle_bits::{
    u1::{b0, u1},
    AsBits,
};
use image::{ImageResult, Rgb, RgbImage};
use num_complex::Complex;
use rand::{thread_rng, RngCore};
use std::time::Instant;

const WIDTH: u16 = 4096;
const HEIGHT: u16 = WIDTH;
const SCALE_X: f64 = 4. / WIDTH as f64;
const SCALE_Y: f64 = 4. / HEIGHT as f64;
const MAX_ITERATIONS: u8 = 255;

fn u8_from_bits(arr: [u1; 3]) -> u8 {
    arr.iter()
        .copied()
        .enumerate()
        .map(|(i, x)| u8::from(x) << i)
        .sum::<u8>()
        * 36
}

fn u8_to_rgb_color(n: u8) -> Rgb<u8> {
    let [a, b, c, d, e, f, g, h] = n.as_bits();
    let red = u8_from_bits([a, b, c]);
    let green = u8_from_bits([d, e, b0]);
    let blue = u8_from_bits([f, g, h]);
    Rgb([red, green, blue])
}

fn main() -> ImageResult<()> {
    let timer = Instant::now();
    let image = RgbImage::from_par_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        let c = Complex::new(
            (x as f64).mul_add(SCALE_X, -2.),
            (y as f64).mul_add(SCALE_Y, -2.),
        );

        let mut z: Complex<f64> = Complex::new(0., 0.);
        let mut i = 0;
        while i < MAX_ITERATIONS && z.norm() <= 2. {
            z = z.powu(2) + c;
            i += 1;
        }
        if i == MAX_ITERATIONS {
            Rgb([0; 3]) // Stable points are black.
        } else {
            u8_to_rgb_color(i) // Unstable points are colored
        }
    });
    println!("Finished simulation in {:?}", timer.elapsed());
    let result: ImageResult<()> =
        image.save(format!("output_images/{}.png", thread_rng().next_u64()));
    println!("Image saved successfully");
    result
}
