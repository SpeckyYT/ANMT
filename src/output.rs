use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime,UNIX_EPOCH};

pub fn manage_output_frames(output_folder: &PathBuf, video_files: Vec<Vec<HashMap<(usize, usize), [u8; 4]>>>) {
    for video_file in video_files {
        output_frames(output_folder, video_file);
    }
}

fn output_frames(output_folder: &PathBuf, video_file: Vec<HashMap<(usize, usize), [u8; 4]>>) {
    // I'm desperate
    let filename = format!("{:0>8}.txt", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() % 100000);
    
    let mut string = String::new();

    // HELP ME, I'M DESPERATE
    string.push_str("20,20,30\n");

    for frame in video_file {
        let mut current_frame = Vec::new();
        for (key, val) in frame.iter() {
            let form = format!("{},{},{},{},{}", key.0, key.1, val[0], val[1], val[2]);
            current_frame.push(form);
        }
        string.push_str(&current_frame.join(":"));
        string.push_str("\n");
    }

    let mut file = std::fs::File::create(output_folder.join(filename)).unwrap();
    file.write_all(string.as_bytes()).expect("Failed to write to file");
}
