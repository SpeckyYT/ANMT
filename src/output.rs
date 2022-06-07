use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

pub fn output_frames(output_folder: &PathBuf, video_file: Vec<HashMap<(usize, usize), [u8; 4]>>, file_name: String) {
    let mut content = String::new();

    // HELP ME, I'M DESPERATE
    content.push_str("20,20,30\n");

    for frame in video_file {
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
