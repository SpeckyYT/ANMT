use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

use crate::Video;

impl Video {
    pub fn extract_frames(&mut self, frames_folder: &PathBuf) {
        let time = Instant::now();

        Command::new("ffmpeg")
        .args([
            "-i",
            self.path.to_str().unwrap(),
            frames_folder.join(format!("{}.%6d.png", self.file_name(""))).to_str().unwrap(),
        ])
        .stdout(if self.quiet { Stdio::null() } else { Stdio::inherit() })
        .stderr(if self.quiet { Stdio::null() } else { Stdio::inherit() })
        .spawn()
        .expect("Failed to execute FFMPEG")
        .wait_with_output()
        .map_err(|err| format!("Error while running FFMPEG (probably caused by unsupported video format): {}", err))
        .unwrap();

        self.extract_time = time.elapsed();
    }
}
