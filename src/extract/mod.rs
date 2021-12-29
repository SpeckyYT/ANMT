use std::path::{PathBuf};
use std::process::{Command, Output};
use std::thread;

pub fn manage_extract_frames(frames_folder: &PathBuf, video_files: &Vec<PathBuf>){
    let mut threads = vec![];
    for video_file in video_files {
        let frames_folder_clone = frames_folder.clone();
        let video_file_clone = video_file.clone();
        threads.push(thread::spawn(move || {
            extract_frames(&frames_folder_clone, &video_file_clone);
        }));
    }
    for current_thread in threads {
        current_thread.join().expect("`manage_extract_frames` thread didn't end successfully.");
    }
}

fn extract_frames(frames_folder: &PathBuf, file_path: &PathBuf) -> Output {
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let current_folder = frames_folder.join(file_name);

    crate::util::summon_folder(&current_folder);

    return Command::new("ffmpeg")
    .args([
        "-i",
        file_path.to_str().unwrap(),
        current_folder.join(
            format!(
                "{}.%{}d.png",
                file_name,
                "6",
            ),
        ).to_str().unwrap(),
    ])
    .spawn()
    .expect("Failed to execute FFMPEG")
    .wait_with_output()
    .expect("Error while running FFMPEG");
}
