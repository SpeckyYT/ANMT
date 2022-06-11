use std::path::Path;
use std::path::PathBuf;

mod extract;
mod process;
mod output;
mod util;

pub struct Video {
    path: PathBuf,
    frames: Vec<Vec<PixelUpdate>>,
    width: usize,
    height: usize,
    fps: f64,
}

pub struct PixelUpdate {
    position: (usize, usize),
    color: [u8; 4],
}

impl Video {
    pub fn new(file_path: PathBuf) -> Video {
        Video {
            path: file_path,
            frames: Vec::new(),
            width: 0,
            height: 0,
            fps: 0.0,
        }
    }
    pub fn log(&self, message: &str, current: usize, total: usize) {
        println!(
            "[{}] {}: {}/{} ({}%)",
            self.file_name(""),
            message,
            current,
            total,
            100 * current / total,
        );
    }
    pub fn file_name(&self, extension: &str) -> String {
        let file_name = self.path.file_stem().unwrap().to_str().unwrap();
        if extension.len() > 0 {
            format!("{}.{}", file_name, extension)
        } else {
            file_name.to_string()
        }
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
    util::summon_folder(&video_folder());
    util::summon_folder(&frames_folder());
    util::summon_folder(&output_folder());

    let video_files = util::find_files(&video_folder(), &SUPPORTED_VIDEO_FORMATS);

    for video_file in video_files {
        let mut video = Video::new(video_file);

        video.extract_frames(&frames_folder())
            .wait_with_output()
            .map_err(|err| format!("Error while running FFMPEG: {}", err))
            .unwrap();

        video.process_frames(&frames_folder());
        video.output_frames(&output_folder());
    }
}

fn video_folder() -> PathBuf {
    return Path::new("videos").to_path_buf();
}

fn frames_folder() -> PathBuf {
    return video_folder().join("frames");
}

fn output_folder() -> PathBuf {
    return video_folder().join("output");
}
