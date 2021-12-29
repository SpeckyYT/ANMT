use std::path::{PathBuf};
use std::thread;
// use image::*;

// copy paste moment (a macro could help?)
pub fn manage_process_frames(frames_folder: &PathBuf, video_files: &Vec<PathBuf>){
    let mut threads = vec![];
    for video_file in video_files {
        let frames_folder_clone = frames_folder.clone();
        let video_file_clone = video_file.clone();
        threads.push(thread::spawn(move || {
            process_frames(&frames_folder_clone, &video_file_clone);
        }));
    }
    for current_thread in threads {
        current_thread.join().expect("`manage_process_frames` thread didn't end successfully.");
    }
}

fn process_frames(frames_folder: &PathBuf, file_path: &PathBuf) {
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let _current_folder = frames_folder.join(file_name);

    // <insert frames preprocessing code here>
}
