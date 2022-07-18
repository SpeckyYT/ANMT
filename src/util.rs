use std::path::Path;
use ffprobe::{ Format, ffprobe };

pub fn ffmpeg_probe(path: &Path) -> Format {
    ffprobe(path).unwrap().format
}
