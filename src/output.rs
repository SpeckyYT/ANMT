use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use std::fs::File;
use crate::PixelUpdate;

use super::Video;

impl Video {
    pub fn output_frames(&mut self, output_folder: &PathBuf) {
        let time = Instant::now();

        let mut txt = String::new();
        let mut anmt = vec![self.width, self.height];

        txt.push_str(format!("{},{},{}\n", self.width, self.height, self.fps).as_str());
        anmt.push(self.fps.floor().clamp(0.0, 255.0) as u8);
        anmt.push((self.fps.fract() * 256.0) as u8);

        for frame in &self.frames {
            let mut txt_current_frame = Vec::new();
            anmt.push(0);

            for PixelUpdate { position, color } in frame {
                txt_current_frame.push(format!("{},{},{},{},{}", position.0, position.1, color[0], color[1], color[2]));
                
                anmt.push(position.0 + 1);
                anmt.push(position.1 + 1);
                anmt.push(color[0]);
                anmt.push(color[1]);
                anmt.push(color[2]);
            }

            txt.push_str(&txt_current_frame.join(":"));
            txt.push_str("\n");
        }
        
        // .txt
        let mut file = File::create(output_folder.join("output.txt")).unwrap();
        file.write_all(txt.as_bytes()).unwrap();

        // .anmt
        let mut file = File::create(output_folder.join(self.file_name("anmt"))).unwrap();
        file.write_all(&anmt).unwrap();

        self.output_time = time.elapsed();
    }
}
