use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

mod extract;
mod process;
mod output;
mod util;

pub struct Video {
    pub path: PathBuf,
    pub frames: Vec<HashMap<(usize,usize), [u8; 4]>>,
    pub width: usize,
    pub height: usize,
}

impl Video {
    pub fn log(&self, message: &str, current: usize, total: usize) {
        println!(
            "{}: {}/{} ({}%)",
            message,
            current,
            total,
            100 * current / total,
        );
    }    
}

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

    for video_file in video_files {
        let mut video = Video {
            path: video_file.clone(),
            frames: vec![],
            width: 0,
            height: 0,
        };

        extract::extract_frames(&frames_folder, &video_file)
            .wait_with_output()
            .map_err(|err| format!("Error while running FFMPEG: {}", err))
            .unwrap();

        video.process_frames(&frames_folder, &video_file);
        video.output_frames(&output_folder);
    }
}
