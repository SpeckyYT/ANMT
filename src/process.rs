use std::path::Path;
use std::time::{Instant, Duration};
use crate::lib::{ Video, PixelUpdate, Color, Optimization };
use crate::util::ffmpeg_probe;

use std::sync::{ Arc, Mutex };
use rayon::prelude::*;
use image::*;

const DEFAULT_FPS: f64 = 24.0;

impl Video {
    pub fn process_frames(&mut self, frames_folder: &Path) -> Duration {
        let time = Instant::now();

        let frames: Vec<_> = frames_folder.read_dir().expect("Failed reading frames directory at process_frames").collect();
        if frames.is_empty() { return time.elapsed(); }
        
        let frame_count = frames.len();
        let first_frame = open_frame(&frames[0].as_ref().unwrap().path());

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
        self.fps = if let Some(duration) = data.duration {
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

        let i = Arc::new(Mutex::new(0));

        let new_frames: Vec<_> = frames.par_iter().map(|frame_path| {
            let frame = open_frame(&frame_path
                .as_ref()
                .expect("Frame reading error")
                .path()
            );
            let resized = frame.resize(width as u32, height as u32, self.filter.to_filter_type());
            let rgb8 = resized.to_rgb8();
            let pixels = rgb8.pixels();

            let mut output = Vec::new();

            for pixel in pixels {
                output.push(flatten_color(&pixel.0, self.color_precision));
            }
            
            let mut lock = i.lock().unwrap();
            *lock += 1;
            self.log_percent("Loaded, resized, processed image", *lock, frames.len());

            output
        }).collect();

        for (index, current_frame) in new_frames.iter().enumerate() {
            let previous_frame = new_frames.get(index - 1);
            let next_frame = new_frames.get(index + 1);
    
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
    
            self.log_percent("Frames optimized and ", index + 1, frame_count);
        }

        time.elapsed()
    }    
}

fn open_frame(path: &Path) -> image::DynamicImage {
    image::open(path)
    .expect("Failed opening frame")
}

fn index_to_position(index: usize, width: usize) -> (usize, usize) {
    let y = index / width;
    let x = index % width;
    (x, y)
}

fn flatten_int(number: u8, bits: u8) -> u8 { // (2^bits + 1) steps
    // if bits >= 8 { return number }
    let max = 2u128.pow(9u32 - bits as u32);
    ((number as f32 / max as f32).round() * max as f32) as u8
}

fn flatten_color(color: &[u8; 3], bits: u8) -> Color {
    Color {
        r: flatten_int(color[0], bits),
        g: flatten_int(color[1], bits),
        b: flatten_int(color[2], bits),
    }
}
