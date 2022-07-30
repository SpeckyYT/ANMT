use std::path::Path;
use ffprobe::{ Format, ffprobe };
use image::DynamicImage;

use crate::data::Color;
// use crate::Color;

#[inline(always)]
pub fn ffmpeg_probe(path: &Path) -> Format {
    ffprobe(path).unwrap().format
}

#[inline(always)]
pub fn open_frame(path: &Path) -> DynamicImage {
    image::open(path)
    .expect("Failed opening frame")
}

#[inline(always)]
pub fn index_to_position(index: usize, width: usize) -> (usize, usize) {
    let y = index / width;
    let x = index % width;
    (x, y)
}

#[inline(always)]
pub fn flatten_int(number: u8, bits: u8) -> u8 { // (2^bits + 1) steps
    // if bits >= 8 { return number }
    let max = 2u128.pow(9u32 - bits as u32);
    ((number as f32 / max as f32).round() * max as f32) as u8
}

#[inline(always)]
pub fn flatten_color(color: &[u8; 3], bits: u8) -> Color {
    Color {
        r: flatten_int(color[0], bits),
        g: flatten_int(color[1], bits),
        b: flatten_int(color[2], bits),
    }
}
