use std::io::Write;
use std::path::PathBuf;
use super::process::Video;

pub fn output_frames(output_folder: &PathBuf, video: Video, file_name: String) {
    let mut content = String::new();

    // I'm not so desperate anymore 😎
    content.push_str(format!("{},{},30\n", video.width, video.height).as_str());

    for frame in video.frames {
        let mut current_frame = Vec::new();
        for (key, val) in frame.iter() {
            let form = format!("{},{},{},{},{}", key.0, key.1, val[0], val[1], val[2]);
            current_frame.push(form);
        }
        content.push_str(&current_frame.join(":"));
        content.push_str("\n");
    }

    let mut file = std::fs::File::create(output_folder.join(file_name)).unwrap();
    file.write_all(content.as_bytes()).expect("Failed to write to file");
}
