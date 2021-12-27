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
};
// use image::*;

const SUPPORTED_VIDEO_FORMATS: [&str; 7] = [
    "mp4",
    "avi",
    "mov",
    "wm",
    "flv",
    "gif",
    "apng",
];

fn main() {
    let video_folder = Path::new("videos");
    let frames_folder_buffer = video_folder.join("frames");
    let frames_folder = frames_folder_buffer.as_path();

    summon_folder(&frames_folder);

    let video_files = find_files(&video_folder, &SUPPORTED_VIDEO_FORMATS);

    println!("{:#?}", video_files);
}

fn summon_folder(folder: &Path) {
    if !folder.is_dir() {
        let backwards = folder.join("..");
        if !backwards.is_dir() { summon_folder(&backwards) }
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
