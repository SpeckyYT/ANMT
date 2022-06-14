use std::path::PathBuf;
use std::fs;
use std::time::Duration;
use clap::Command;
use clap::ValueHint;
use clap::arg;

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
    quiet: bool,
    extract_time: Duration,
    process_time: Duration,
    output_time: Duration,
}

pub struct PixelUpdate {
    position: (u8, u8),
    color: [u8; 4],
}

impl Video {
    pub fn new(file_path: &PathBuf, quiet: bool) -> Video {
        Video {
            path: file_path.to_path_buf(),
            frames: Vec::new(),
            width: 0,
            height: 0,
            duration: 0.0,
            fps: 0.0,
            quiet: quiet,
            extract_time: Duration::new(0, 0),
            process_time: Duration::new(0, 0),
            output_time: Duration::new(0, 0),
        }
    }
    pub fn log(&self, message: String) {
        if !self.quiet {
            println!("[{}] {}", self.file_name(""), message);
        }
    }
    pub fn log_percent(&self, message: &str, current: usize, total: usize) {
        self.log(
            format!(
                "{}: {}/{} ({}%)",
                message,
                current,
                total,
                100 * current / total,
            )
        );
    }
    pub fn log_final(&self) {
        self.log(format!("{} frames", self.frames.len()));
        self.log(format!("{} frames per second", self.fps));
        self.log(format!("{} seconds duration", self.duration));
        self.log(format!("{} pixels ({}x{})", self.width as u16 * self.height as u16, self.width, self.height));
        self.log(format!("{} color change triggers", self.frames.iter().map(|f| f.len()).sum::<usize>()));
        self.log(format!("{}s extract time", self.extract_time.as_secs_f64()));
        self.log(format!("{}s process time", self.process_time.as_secs_f64()));
        self.log(format!("{}s output time", self.output_time.as_secs_f64()));
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

fn main() {
    let matches = Command::new("ANMT")
    .arg_required_else_help(true)
    .args(&[
        arg!(<VIDEO_FILE> "Video file to process").value_hint(ValueHint::FilePath),
        arg!(-q --quiet "Don't print anything"),
    ])
    .get_matches();

    let video_file = matches.value_of("VIDEO_FILE").unwrap();
    let quiet = matches.is_present("quiet");

    let video_file = PathBuf::from(video_file);
    let video_file = if video_file.is_relative() { std::env::current_dir().unwrap().join(video_file) } else { video_file };

    let mut video = Video::new(&video_file, quiet);

    if !video_file.exists() { panic!("Path does not exist") }
    if !video_file.is_file() { panic!("Path is not a file") }

    let filename = video_file.file_stem().unwrap().to_str().unwrap();
    let video_folder = video_file.parent().unwrap().to_path_buf();
    let frames_folder = video_folder.join(filename);

    // create `frames_folder` folder
    if fs::create_dir_all(&frames_folder).is_err() {
        panic!("Could not create folder: `{}`", frames_folder.to_str().unwrap());
    }

    video.extract_frames(&frames_folder, quiet);
    video.process_frames(&frames_folder);
    video.output_frames(&video_folder);

    video.log_final();
}
