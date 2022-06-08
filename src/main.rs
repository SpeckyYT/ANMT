use std::path::Path;

mod extract;
mod process;
mod output;
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

    for video_file in video_files {
        let file_name = video_file.file_name().unwrap().to_str().unwrap();
        // remove characters that are not allowed in filenames (all in one line)
        let file_name = match file_name.replace(|c: char| !c.is_ascii_alphanumeric(), "") {
            a if a == String::from("") => String::from("video"),
            a => a,
        };

        extract::extract_frames(&frames_folder, &video_file)
            .wait_with_output()
            .map_err(|err| format!("Error while running FFMPEG: {}", err))
            .unwrap();

        let video = process::process_frames(&frames_folder, &video_file);

        output::output_frames(&output_folder, video, file_name);
    }
}
