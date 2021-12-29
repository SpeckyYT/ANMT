use std::path::{Path};

mod extract;
mod process;
mod util;

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

    util::summon_folder(&video_folder);
    util::summon_folder(&frames_folder);
    util::summon_folder(&output_folder);

    let video_files = util::find_files(&video_folder, &SUPPORTED_VIDEO_FORMATS);

    extract::manage_extract_frames(&frames_folder, &video_files);
    process::manage_process_frames(&frames_folder, &video_files);
}
