use std::path::PathBuf;
use std::process::{Command, Child};
use super::Video;

impl Video {
    pub fn extract_frames(&self, frames_folder: &PathBuf) -> Child {
        let current_folder = frames_folder.join(self.file_name(""));

        crate::util::summon_folder(&current_folder);

        return Command::new("ffmpeg")
        .args([
            "-i",
            self.path.to_str().unwrap(),
            current_folder.join(
                format!(
                    "{}.%{}d.png",
                    self.file_name(""),
                    "6",
                ),
            ).to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to execute FFMPEG");
    }
}
