mod extract;
mod process;
mod output;
mod util;
mod lib;

use std::path::PathBuf;
use std::fs;
use clap::Command;
use clap::ValueHint;
use clap::arg;
use image::imageops::FilterType;
use lib::{ Video, Optimization };

fn main() {
    let matches = Command::new("ANMT")
    .arg_required_else_help(true)
    .args(&[
        arg!(<VIDEO_FILE> "Video file to process").value_hint(ValueHint::FilePath),
        arg!(-q --quiet "Don't print anything"),
        arg!(-e --skip_extract "Skip extraction"),
        arg!(-o --optimize [optimization] "Optimize the video (none, forward, backward, both)"),
        arg!(-f --filter [filter] "Filter the video (nearest, linear, cubic, gaussian, lanczos3)"),
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
        None => Optimization::Forward,
    };
    let filter = match matches.value_of("filter") {
        Some(f) => match f {
            "nearest" => FilterType::Nearest,
            "linear" => FilterType::Triangle,
            "cubic" => FilterType::CatmullRom,
            "gaussian" => FilterType::Gaussian,
            "lanczos3" => FilterType::Lanczos3,
            _ => panic!("Unknown filter"),
        },
        None => FilterType::Triangle,
    };

    let mut video = Video::new(&video_file, quiet, skip_extract, optimization, filter);

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
        video.extract_time = video.extract_frames(&frames_folder);
    }
    video.process_time = video.process_frames(&frames_folder);
    video.output_time = video.output_frames(&video_folder);

    video.log_final();
}
