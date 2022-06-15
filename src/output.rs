use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use std::fs::File;
use json::{object, array};

use crate::PixelUpdate;
use crate::Video;

impl Video {
    pub fn output_frames(&mut self, output_folder: &PathBuf) {
        let time = Instant::now();

        let mut txt = String::new();
        txt.push_str(format!("{},{},{}\n", self.width, self.height, self.fps).as_str());

        let mut anmt = vec![
            self.width,
            self.height,
            self.fps.floor().clamp(0.0, 255.0) as u8,
            (self.fps.fract() * 256.0) as u8,
        ];

        let mut json = object!{
            "width" => self.width,
            "height" => self.height,
            "fps" => self.fps,
            "frames" => array![],
        };

        for frame in &self.frames {
            let mut txt_current_frame = Vec::new();
            anmt.push(0);
            json["frames"].push(array![]).unwrap();

            for PixelUpdate { position, color } in frame {
                txt_current_frame.push(format!("{},{},{},{},{}", position.0, position.1, color[0], color[1], color[2]));
                
                anmt.push(position.0 + 1);
                anmt.push(position.1 + 1);
                anmt.push(color[0]);
                anmt.push(color[1]);
                anmt.push(color[2]);

                let json_last_frame = json["frames"].len()-1;
                json["frames"][json_last_frame].push(
                    array![
                        position.0,
                        position.1,
                        color[0],
                        color[1],
                        color[2],
                    ]
                ).unwrap();
            }

            txt.push_str(&txt_current_frame.join(":"));
            txt.push_str("\n");
        }
        
        // .txt
        write_file_u8(
            &output_folder.join(self.file_name("txt")),
            txt.as_bytes(),
        );
        // .anmt
        write_file_u8(
            &output_folder.join(self.file_name("anmt")),
            &anmt,
        );
        // .json
        write_file_u8(
            &output_folder.join(self.file_name("json")),
            json.to_string().as_bytes(),
        );

        self.output_time = time.elapsed();
    }
}

fn write_file_u8(file: &PathBuf, value: &[u8]) {
    File::create(file).unwrap().write_all(value).unwrap();
}
