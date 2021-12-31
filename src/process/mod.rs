use std::path::{PathBuf};
use std::thread;

use image::*;

pub fn manage_process_frames(frames_folder: &PathBuf, video_files: &Vec<PathBuf>){
    for video_file in video_files {
        process_frames(&frames_folder, &video_file);
    }
}

fn process_frames(frames_folder: &PathBuf, file_path: &PathBuf) {
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let current_folder = frames_folder.join(file_name);

    let frames: Vec<_> = current_folder.read_dir().expect("Failed reading frames directory at process_frames").collect();

    let first_frame = image::open(frames[0].unwrap().path()).unwrap();

    let (o_width, o_height) = first_frame.dimensions();
    let o_pixels: f64 = (o_width as f64) * (o_height as f64);
    
    let ratio: f64 = (o_width as f64) / (o_height as f64);
    let scale: f64 = (o_pixels / 999_f64).sqrt();
    
    let width: u32 = (ratio * (o_width as f64) / scale) as u32;
    let height: u32 = ((o_height as f64) / scale) as u32;

    for frame_index in 0..frames.len() {
        let current_frame = &frames[frame_index];
        let frame_entry = current_frame.as_ref().unwrap();
        thread::spawn(move || {
            let frame_path = frame_entry.path();
            
            let frame = image::open(frame_path).expect("Failed to read frame");

            let resized = frame.resize(width, height, imageops::FilterType::Nearest);

            let stuff = resized.as_rgb8().unwrap();

            println!("Frame #{} done", frame_index)
        });
    }
}
