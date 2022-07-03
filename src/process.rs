use std::path::PathBuf;
use std::time::Instant;

use crate::{ Video, PixelUpdate, Optimization };
use crate::util::ffmpeg_probe;

use image::*;

const COLOR_PRECISION: u8 = 6;
const DEFAULT_FPS: f64 = 24.0;
const MAX_PIXELS: f64 = 999.0;

impl Video {
    pub fn process_frames(&mut self, frames_folder: &PathBuf, optimize: Optimization) {
        let time = Instant::now();
    
        let frames: Vec<_> = frames_folder.read_dir().expect("Failed reading frames directory at process_frames").collect();
        let frames: Vec<_> = frames.iter().map(|f|
            f.as_ref().map_err(|err| format!("Frame reading error: {}", err)).unwrap()
        ).collect();
    
        let frame_count = frames.len();
    
        let first_frame = image::open(frames[0].path()).unwrap();
        
        // resize first_frame.dimensions() that it uses 999 total pixels or less
        let (width, height) = first_frame.dimensions();
        let surface = (width as f64) * (height as f64);
        let ratio = surface / MAX_PIXELS as f64;
        let (width, height) = if ratio <= 1.0 {
            (width as u8, height as u8)
        } else {
            let scale = 1.0 / ratio.sqrt();
            ((width as f64 * scale).floor() as u8, (height as f64 * scale).floor() as u8)
        };

        self.width = width;
        self.height = height;

        let data = ffmpeg_probe(&self.path);
        
        let mut default_fps = || {
            self.duration = (frame_count as f64) / DEFAULT_FPS;
            DEFAULT_FPS
        };

        self.fps = if data.duration.is_some() {
            let duration = data.duration.unwrap();
            let duration: Result<f64, _> = duration.parse();
            if duration.is_ok() {
                let duration = duration.unwrap();
                self.duration = duration;
                1000.0 * duration / (frame_count as f64)
            } else {
                default_fps()
            }
        } else {
            default_fps()
        };

        let mut new_frames = vec![];

        for frame_index in 0..frame_count {
            let frame_entry = frames[frame_index].path();
            let frame_path = frame_entry;
            let frame = image::open(frame_path).expect("Failed to read frame");
            let resized = frame.resize(width as u32, height as u32, imageops::FilterType::Nearest);
            let resized = resized.to_rgba8();
            let pixels = resized.pixels();

            let mut output: Vec<[u8; 4]> = Vec::new();
            for pixel in pixels {
                output.push(flatten_color(&pixel.0, COLOR_PRECISION));
            }
            new_frames.push(output);
            self.log_percent("Frames resized", frame_index + 1, frame_count);
        }

        for fr in 0..new_frames.len() {
            let current_frame = &new_frames[fr];
            let previous_frame = new_frames.get(fr - 1);
            let next_frame = new_frames.get(fr + 1);
    
            let mut changes = Vec::new();
    
            for i in 0..current_frame.len() {
                match optimize {
                    Optimization::None => (),
                    Optimization::Forward => {
                        if previous_frame.is_some() {
                            let previous_frame = previous_frame.unwrap();
                            if current_frame[i] == previous_frame[i] { continue }
                        }
                    },
                    Optimization::Backward => {
                        if next_frame.is_some() {
                            let next_frame = next_frame.unwrap();
                            if current_frame[i] == next_frame[i] { continue }
                        }
                    },
                    Optimization::Both => {
                        if previous_frame.is_some() && next_frame.is_some() {
                            let previous_frame = previous_frame.unwrap();
                            let next_frame = next_frame.unwrap();
                            if current_frame[i] == previous_frame[i] && current_frame[i] == next_frame[i] { continue }
                        }
                    }
                }
                let (x, y) = index_to_position(i, self.width.into());
                changes.push(PixelUpdate { position: (x as u8, y as u8), color: current_frame[i] });
            }
    
            self.frames.push(changes);
    
            self.log_percent("Frames processed", fr + 1, frame_count);
        }

        self.process_time = time.elapsed();
    }    
}

fn index_to_position(index: usize, width: usize) -> (usize, usize) {
    let y = index / width;
    let x = index % width;
    (x, y)
}

fn flatten_int(number: u8, bits: u8) -> u8 {
    let bits = 2_u128.pow(bits.into());
    ((number as f64 / bits as f64).round() * bits as f64) as u8
}

fn flatten_color(color: &[u8; 4], bits: u8) -> [u8; 4] {
    let mut output = [color[3]; 4];
    output[0] = flatten_int(color[0], bits);
    output[1] = flatten_int(color[1], bits);
    output[2] = flatten_int(color[2], bits);
    return output;
}
