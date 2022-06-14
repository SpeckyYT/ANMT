use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

use crate::Video;

impl Video {
    pub fn extract_frames(&mut self, frames_folder: &PathBuf) {
        let time = Instant::now();

        let current_folder = frames_folder.join(self.file_name(""));

        crate::util::summon_folder(&current_folder);

        Command::new("ffmpeg")
        .args([
            "-i",
            self.path.to_str().unwrap(),
            current_folder.join(format!("{}.%6d.png", self.file_name(""))).to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to execute FFMPEG")
        .wait_with_output()
        .map_err(|err| format!("Error while running FFMPEG: {}", err))
        .unwrap();

        self.extract_time = time.elapsed();
    }
}
