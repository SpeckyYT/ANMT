use std::path::Path;
use std::time::{Instant, Duration};
use crate::data::{ Video, PixelUpdate, Optimization };
use crate::util::ffmpeg_probe;
use crate::util::open_frame;
use crate::util::flatten_color;

use std::sync::{ Arc, Mutex };
use rayon::prelude::*;
use image::GenericImageView;

const DEFAULT_FPS: f64 = 24.0;

impl Video {
    pub fn process_frames(&mut self, frames_folder: &Path) -> (Vec<Vec<PixelUpdate>>, Duration) {
        let time = Instant::now();

        let frames: Vec<_> = frames_folder.read_dir().expect("Failed reading frames directory at process_frames").collect();
        if frames.is_empty() { return (Vec::default(), time.elapsed()); }
        self.frame_count = frames.len();

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
            self.duration = (self.frame_count as f64) / DEFAULT_FPS;
            DEFAULT_FPS
        };
        self.fps = if let Some(duration) = data.duration {
            let duration: Result<f64, _> = duration.parse();
            if let Ok(duration) = duration {
                self.duration = duration;
                self.frame_count as f64 / duration
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
            let resized = frame.resize_exact(self.width as u32, self.height as u32, self.filter.to_filter_type());
            let rgb8 = resized.to_rgb8();
            let pixels = rgb8.pixels();
            let output = pixels.enumerate().par_bridge()
                .map(
                    |(i, pixel)|
                    flatten_color(&pixel.0, self.color_precision)
                    .to_pixel_update(i, self.width)
                ).collect::<Vec<_>>();
            
            let mut lock = i.lock().unwrap();
            *lock += 1;
            self.log_percent("Loaded, resized, processed image", *lock, self.frame_count);

            output
        }).collect();

        (new_frames, time.elapsed())
    }
    pub fn optimize_frames(&mut self, frames: Vec<Vec<PixelUpdate>>) -> (Vec<Vec<PixelUpdate>>, Duration) {
        let time = Instant::now();

        let changes = frames.iter().enumerate().map(|(i, current_frame)| {
            let previous_frame = frames.get(i - 1);
            let next_frame = frames.get(i + 1);

            let mut changes = Vec::new();

            for (i, current_pixel) in current_frame.iter().enumerate() {
                match self.optimization {
                    Optimization::None => (),
                    Optimization::Forward => {
                        if let Some(previous_frame) = previous_frame {
                            if current_pixel == &previous_frame[i] { continue }
                        }
                    },
                    Optimization::Backward => {
                        if let Some(next_frame) = next_frame {
                            if current_pixel == &next_frame[i] { continue }
                        }
                    },
                    Optimization::Both => {
                        if let (Some(previous_frame), Some(next_frame)) = (previous_frame, next_frame) {
                            if current_pixel == &previous_frame[i] && current_pixel == &next_frame[i] { continue }
                        }
                    }
                }
                changes.push(*current_pixel);
            }

            self.log_percent("Frames optimized", i + 1, self.frame_count);

            changes
        }).collect();

        (changes, time.elapsed())
    }    
}
