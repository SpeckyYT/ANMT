use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use crate::PixelUpdate;

use super::Video;

impl Video {
    pub fn output_frames(&mut self, output_folder: &PathBuf) {
        let time = Instant::now();

        let mut content = String::new();

        // I'm not so desperate anymore 😎
        content.push_str(format!("{},{},{}\n", self.width, self.height, self.fps).as_str());

        for frame in &self.frames {
            let mut current_frame = Vec::new();
            for PixelUpdate { position, color } in frame {
                let form = format!("{},{},{},{},{}", position.0, position.1, color[0], color[1], color[2]);
                current_frame.push(form);
            }
            content.push_str(&current_frame.join(":"));
            content.push_str("\n");
        }
        
        let mut file = std::fs::File::create(output_folder.join(self.file_name("txt"))).unwrap();
        file.write_all(content.as_bytes()).expect("Failed to write to file");

        self.output_time = time.elapsed();
    }
}
