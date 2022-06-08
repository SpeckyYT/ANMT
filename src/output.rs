use std::io::Write;
use std::path::PathBuf;
use super::Video;

impl Video {
    pub fn output_frames(&self, output_folder: &PathBuf) {
        let mut content = String::new();

        // I'm not so desperate anymore 😎
        content.push_str(format!("{},{},30\n", self.width, self.height).as_str());

        for frame in &self.frames {
            let mut current_frame = Vec::new();
            for (key, val) in frame.iter() {
                let form = format!("{},{},{},{},{}", key.0, key.1, val[0], val[1], val[2]);
                current_frame.push(form);
            }
            content.push_str(&current_frame.join(":"));
            content.push_str("\n");
        }

        
        let mut file = std::fs::File::create(output_folder.join(self.file_name("txt"))).unwrap();
        file.write_all(content.as_bytes()).expect("Failed to write to file");
    }
}
