use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

mod extract;
mod process;
mod output;
mod util;

pub struct Video {
    path: PathBuf,
    frames: Vec<Vec<PixelUpdate>>,
    width: u8, // atm u8 are enough
    height: u8,
    duration: f64,
    fps: f64,
    extract_time: Duration,
    process_time: Duration,
    output_time: Duration,
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
            duration: 0.0,
            fps: 0.0,
            extract_time: Duration::new(0, 0),
            process_time: Duration::new(0, 0),
            output_time: Duration::new(0, 0),
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
    pub fn log_simple(&self, message: String) {
        println!("[{}] {}", self.file_name(""), message);
    }
    pub fn log_final(&self) {
        self.log_simple(format!("{} frames", self.frames.len()));
        self.log_simple(format!("{} frames per second", self.fps));
        self.log_simple(format!("{} seconds duration", self.duration));
        self.log_simple(format!("{} pixels ({}x{})", self.width * self.height, self.width, self.height));
        self.log_simple(format!("{} color change triggers", self.frames.iter().map(|f| f.len()).sum::<usize>()));
        self.log_simple(format!("{}s extract time", self.extract_time.as_secs_f64()));
        self.log_simple(format!("{}s process time", self.process_time.as_secs_f64()));
        self.log_simple(format!("{}s output time", self.output_time.as_secs_f64()));
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

        video.extract_frames(&frames_folder());
        video.process_frames(&frames_folder());
        video.output_frames(&output_folder());

        video.log_final();
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
