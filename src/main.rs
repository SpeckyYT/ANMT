use std::{
    ffi::{
        OsStr,
    },
    fs,
    iter::{
        Iterator,
    },
    path::{
        Path, PathBuf,
    },
    process::{
        Command,
        Output,
    },
    thread,
};
// use image::*;

const SUPPORTED_VIDEO_FORMATS: [&str; 9] = [
    "apng",
    "avi",
    "flv",
    "gif",
    "mkv",
    "mov",
    "mp4",
    "mvi",
    "wmv",
];

fn main() {
    let video_folder = Path::new("videos").to_path_buf();
    let frames_folder = video_folder.join("frames");
    let output_folder = video_folder.join("output");

    summon_folder(&video_folder);
    summon_folder(&frames_folder);
    summon_folder(&output_folder);

    let video_files = find_files(&video_folder, &SUPPORTED_VIDEO_FORMATS);

    manage_extract_frames(&frames_folder, &video_files);
    manage_process_frames(&frames_folder, &video_files);
}

fn summon_folder(folder: &Path) {
    if !folder.exists() {
        let backwards = folder.join("..");
        if !backwards.exists() { summon_folder(&backwards) }
        fs::create_dir(folder).unwrap();
    }
}

fn find_files(folder: &Path, extensions: &[&str]) -> Vec<PathBuf> {
    return folder
    .read_dir()
    .unwrap()
    .filter(|entry| {
        let dir_entry = entry.as_ref().unwrap().to_owned();
        let path = dir_entry.path();
        if path.is_dir() { return false }
        let ext = path.extension().unwrap_or(OsStr::new("")).to_str().unwrap();
        return extensions.contains(&ext);
    })
    .map(|a| a.unwrap().path())
    .collect();
}

fn manage_extract_frames(frames_folder: &PathBuf, video_files: &Vec<PathBuf>){
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

    summon_folder(&current_folder);

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

// copy paste moment (a macro could help?)
fn manage_process_frames(frames_folder: &PathBuf, video_files: &Vec<PathBuf>){
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
    let current_folder = frames_folder.join(file_name);

    // <insert frames preprocessing code here>
}
