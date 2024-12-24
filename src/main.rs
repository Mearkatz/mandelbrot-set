#![allow(clippy::cast_lossless)]

use beetle_bits::{u1, AsBits};
use image::{ImageResult, Rgb, RgbImage};
use num::{Complex, Zero};
use rand::{thread_rng, Rng};
use std::time::Instant;

const WIDTH: u16 = 4096;
const HEIGHT: u16 = WIDTH;
const SCALE_X: f64 = 4. / WIDTH as f64;
const SCALE_Y: f64 = 4. / HEIGHT as f64;

const MAX_ITERATIONS: u8 = 255;

fn random_string(length: usize) -> String {
    (0..length).fold(String::new(), |s, _| {
        format!("{s}{:x}", thread_rng().gen::<u64>())
    })
}

#[allow(clippy::cast_possible_truncation)]
fn u8_from_bits(arr: &[u1::u1]) -> u8 {
    let it = arr.iter().copied().enumerate();
    let sum: u8 = it.map(|(i, x)| u8::from(x) << i).sum();
    sum * (255 / (1 << arr.len()))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn u16_to_rgb_color(n: u16) -> Rgb<u8> {
    let bits: Vec<u1::u1> = n.to_ne_bytes().map(|x| x.as_bits()).concat();
    let bits: [u1::u1; 16] = bits.try_into().unwrap();

    // Green only gets two bits of color information, which means in theory the image will appear more blue and red, and green will be more sparse
    let red = u8_from_bits(&bits[..6]);
    let green = u8_from_bits(&bits[6..11]);
    let blue = u8_from_bits(&bits[11..]);
    Rgb([red, green, blue])
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn u8_to_rgb_color(n: u8) -> Rgb<u8> {
    let bits = n.as_bits();

    // Green only gets two bits of color information, which means in theory the image will appear more blue and red, and green will be more sparse
    let red = u8_from_bits(&bits[..3]);
    let green = u8_from_bits(&bits[3..6]);
    let blue = u8_from_bits(&bits[6..]);
    Rgb([red, green, blue])
}

fn invert_rgb_color(color: Rgb<u8>) -> Rgb<u8> {
    Rgb(color.0.map(|x| 255 - x))
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::cast_precision_loss
)]
fn main() -> ImageResult<()> {
    let timer = Instant::now();
    let image = RgbImage::from_par_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        let mut c = Complex::new(
            (x as f64).mul_add(SCALE_X, -2.),
            (y as f64).mul_add(SCALE_Y, -2.),
        );

        let mut z: Complex<f64> = Complex::zero();
        let mut i = 0;

        while i < MAX_ITERATIONS && z.norm() <= 2. {
            // z = z.powu(2) + c;
            z = z.powf(2.1) + c;
            c = c.scale(1.1);
            i += 1;
        }

        // Stable points are black.
        if i == MAX_ITERATIONS {
            Rgb([0; 3])
        } else if i > 1 {
            u8_to_rgb_color(i)
            // Rgb([255 - (i as u8); 3])
        } else {
            Rgb([0; 3])
        }
    });
    println!("Finished simulation in {:?}", timer.elapsed());
    let timer = Instant::now();
    let result = image.save(format!("output_images/{}.png", random_string(1)));
    println!("Saved image in {:?}", timer.elapsed());
    result
}
