pub mod data;
pub mod extract;
pub mod process;
pub mod output;
pub mod util;

use std::path::PathBuf;
use std::fs;
use clap::Command;
use clap::ValueHint;
use clap::arg;
use data::{ Video, Optimization, Filter };

const DEFAULT_OPTIMIZATION: Optimization = Optimization::Forward;
const DEFAULT_FILTER: Filter = Filter::Linear;
const DEFAULT_COLOR_PRECISION: u8 = 6;
const DEFAULT_PIXELS: u32 = 999;

fn main() {
    let matches = Command::new("ANMT")
    .arg_required_else_help(true)
    .args(&[
        arg!(<VIDEO_FILE> "Video file to process").value_hint(ValueHint::FilePath),
        arg!(-q --quiet "Don't print anything"),
        arg!(-e --skip_extract "Skip extraction"),
        arg!(-o --optimize [optimization] "Optimize the video (none, forward, backward, both)"),
        arg!(-f --filter [filter] "Filter the video (nearest, linear, cubic, gaussian, lanczos3)"),
        arg!(-b --bits [bits] "Color precision (1-8) (8 is best)"),
        arg!(-p --pixels [pixels] "Maximum number of pixels to output"),
    ])
    .get_matches();

    let video_file = matches.value_of("VIDEO_FILE").unwrap();
    let video_file = PathBuf::from(video_file);
    let video_file = if video_file.is_relative() { std::env::current_dir().unwrap().join(video_file) } else { video_file };

    let quiet = matches.is_present("quiet");
    let skip_extract = matches.is_present("skip_extract");
    let optimization = match matches.value_of("optimize") {
        Some(o) => match o {
            "none" => Optimization::None,
            "forward" => Optimization::Forward,
            "backward" => Optimization::Backward,
            "both" => Optimization::Both,
            _ => panic!("Unknown optimization"),
        },
        None => DEFAULT_OPTIMIZATION,
    };
    let filter = match matches.value_of("filter") {
        Some(f) => match f {
            "nearest" => Filter::Nearest,
            "linear" => Filter::Linear,
            "cubic" => Filter::Cubic,
            "gaussian" => Filter::Gaussian,
            "lanczos3" => Filter::Lanczos3,
            _ => panic!("Unknown filter"),
        },
        None => DEFAULT_FILTER,
    };
    let color_precision = match matches.value_of("bits") {
        Some(b) => match b.parse() {
            Ok(b) if (1..=8).contains(&b) => b,
            _ => DEFAULT_COLOR_PRECISION,
        },
        None => DEFAULT_COLOR_PRECISION,
    };
    let max_pixels = match matches.value_of("pixels") {
        Some(m) => m.parse().unwrap_or(DEFAULT_PIXELS),
        None => DEFAULT_PIXELS,
    };

    let mut video = Video::new(
        &video_file,
        quiet,
        skip_extract,
        optimization,
        filter,
        color_precision,
        max_pixels,
    );

    if !video_file.exists() { panic!("Path does not exist") }
    if !video_file.is_file() { panic!("Path is not a file") }

    let filename = video_file.file_stem().unwrap().to_str().unwrap();
    let video_folder = video_file.parent().unwrap().to_path_buf();
    let frames_folder = video_folder.join(filename);

    // create `frames_folder` folder
    if fs::create_dir_all(&frames_folder).is_err() {
        panic!("Could not create folder: `{}`", frames_folder.to_str().unwrap());
    }

    if !skip_extract {
        video.time.push(("extract", video.extract_frames(&frames_folder)))
    }

    let (frames, time) = video.process_frames(&frames_folder);
    video.time.push(("process", time));
    
    let (changes, time) = video.optimize_frames(frames);
    video.frames = changes;
    video.time.push(("optimizing", time));

    video.time.push(("output txt", video.output_txt(&video_folder)));
    video.time.push(("output anmt", video.output_anmt(&video_folder)));
    video.time.push(("output json", video.output_json(&video_folder)));

    video.log_final();
}
