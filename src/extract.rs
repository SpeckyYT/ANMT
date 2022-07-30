use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;

use crate::Video;

impl Video {
    pub fn extract_frames(&self, frames_folder: &Path) -> Duration {
        let time = Instant::now();

        let stdio_inherit = || if self.quiet { Stdio::null() } else { Stdio::inherit() };

        Command::new("ffmpeg")
        .args([
            "-i",
            self.path.to_str().unwrap(),
            frames_folder.join(format!("{}.%6d.png", self.file_name(""))).to_str().unwrap(),
        ])
        .stdout(stdio_inherit())
        .stderr(stdio_inherit())
        .spawn()
        .expect("Failed to execute FFMPEG")
        .wait_with_output()
        .expect("FFMPEG failed (probably unsupported video format)");

        time.elapsed()
    }
}
