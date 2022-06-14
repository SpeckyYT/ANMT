use std::path::PathBuf;
use ffprobe::{ Format, ffprobe };

pub fn ffmpeg_probe(path: &PathBuf) -> Format {
    ffprobe(path).unwrap().format
}
