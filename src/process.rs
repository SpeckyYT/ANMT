use std::path::Path;
use std::time::{Instant, Duration};
use crate::lib::{ Video, PixelUpdate, Optimization };
use crate::util::ffmpeg_probe;

use image::*;

const DEFAULT_FPS: f64 = 24.0;

impl Video {
    pub fn process_frames(&mut self, frames_folder: &Path) -> Duration {
        let time = Instant::now();

        let frames: Vec<_> = frames_folder.read_dir().expect("Failed reading frames directory at process_frames").collect();
        let frames: Vec<_> = frames.iter().map(|f|
            f.as_ref().map_err(|err| format!("Frame reading error: {}", err)).unwrap()
        ).collect();
    
        let frame_count = frames.len();
    
        let first_frame = image::open(frames[0].path()).unwrap();

        let (width, height) = first_frame.dimensions();
        let surface = (width as f64) * (height as f64);
        let ratio = surface / self.max_pixels as f64;
        let (width, height) = if ratio <= 1.0 {
            (width as usize, height as usize)
        } else {
            let scale = 1.0 / ratio.sqrt();
            ((width as f64 * scale).floor() as usize, (height as f64 * scale).floor() as usize)
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
            if let Ok(duration) = duration {
                self.duration = duration;
                frame_count as f64 / duration
            } else {
                default_fps()
            }
        } else {
            default_fps()
        };

        let mut new_frames = vec![];

        for (frame_index, frame_entry) in frames.iter().enumerate().take(frame_count) {
            let frame_path = frame_entry.path();
            let frame = image::open(frame_path).expect("Failed to read frame");
            let resized = frame.resize(width as u32, height as u32, self.filter.to_filter_type());
            let resized = resized.to_rgba8();
            let pixels = resized.pixels();

            let mut output: Vec<[u8; 4]> = Vec::new();
            for pixel in pixels {
                output.push(flatten_color(&pixel.0, self.color_precision));
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
                match self.optimization {
                    Optimization::None => (),
                    Optimization::Forward => {
                        if let Some(previous_frame) = previous_frame {
                            if current_frame[i] == previous_frame[i] { continue }
                        }
                    },
                    Optimization::Backward => {
                        if let Some(next_frame) = next_frame {
                            if current_frame[i] == next_frame[i] { continue }
                        }
                    },
                    Optimization::Both => {
                        if let (Some(previous_frame), Some(next_frame)) = (previous_frame, next_frame) {
                            if current_frame[i] == previous_frame[i] && current_frame[i] == next_frame[i] { continue }
                        }
                    }
                }
                let (x, y) = index_to_position(i, self.width);
                changes.push(PixelUpdate { position: (x as u8, y as u8), color: current_frame[i] });
            }
    
            self.frames.push(changes);
    
            self.log_percent("Frames processed", fr + 1, frame_count);
        }

        time.elapsed()
    }    
}

fn index_to_position(index: usize, width: usize) -> (usize, usize) {
    let y = index / width;
    let x = index % width;
    (x, y)
}

fn flatten_int(number: u8, bits: u8) -> u8 { // (2^bits + 1) steps
    if bits >= 8 { return number }
    let max = 2u128.pow(9u32 - bits as u32);
    ((number as f32 / max as f32).round() * max as f32) as u8
}

fn flatten_color(color: &[u8; 4], bits: u8) -> [u8; 4] {
    [
        flatten_int(color[0], bits),
        flatten_int(color[1], bits),
        flatten_int(color[2], bits),
        color[3],
    ]
}
