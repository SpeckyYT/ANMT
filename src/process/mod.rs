use std::path::{PathBuf};
// use std::thread;

extern crate image;
// use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

pub fn manage_process_frames(frames_folder: &PathBuf, video_files: &Vec<PathBuf>){
    for video_file in video_files {
        process_frames(&frames_folder, &video_file);
    }
}

fn process_frames(frames_folder: &PathBuf, file_path: &PathBuf) {
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let _current_folder = frames_folder.join(file_name);

    // <insert frames preprocessing code here>
}
