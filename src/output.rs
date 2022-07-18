use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Instant, Duration};
use std::fs::File;
use json::{object, array, JsonValue};

use crate::lib::PixelUpdate;
use crate::Video;

impl Video {
    pub fn output_frames(&self, output_folder: &Path) -> Duration {
        let time = Instant::now();

        // .txt
        let txt = convert_to_txt(self);
        write_file_u8(
            &output_folder.join(self.file_name("txt")),
            txt.as_bytes(),
        );

        // .anmt
        let anmt = convert_to_anmt(self);
        write_file_u8(
            &output_folder.join(self.file_name("anmt")),
            &anmt,
        );

        // .json
        let json = convert_to_json(self);
        write_file_u8(
            &output_folder.join(self.file_name("json")),
            json.to_string().as_bytes(),
        );

        time.elapsed()
    }
}

fn convert_to_txt(video: &Video) -> String {
    let mut txt = String::new();
    txt.push_str(format!("{},{},{}\n", video.width, video.height, video.fps).as_str());
    for frame in &video.frames {
        let mut txt_current_frame = Vec::new();
        for PixelUpdate { position, color } in frame {
            txt_current_frame.push(format!("{},{},{},{},{}", position.0, position.1, color[0], color[1], color[2]));
        }
        txt.push_str(&txt_current_frame.join(":"));
        txt.push('\n');
    }
    txt
}

fn convert_to_anmt(video: &Video) -> Vec<u8> {
    let mut anmt = vec![
        video.width as u8,
        video.height as u8,
        video.fps.floor().clamp(0.0, 255.0) as u8,
        (video.fps.fract() * 256.0) as u8,
    ];

    for frame in &video.frames {
        anmt.push(0);
        for PixelUpdate { position, color } in frame {
            anmt.push(position.0 + 1);
            anmt.push(position.1 + 1);
            anmt.push(color[0]);
            anmt.push(color[1]);
            anmt.push(color[2]);
        }
    }
    anmt
}

fn convert_to_json(video: &Video) -> JsonValue {
    let mut json = object!{
        "width" => video.width,
        "height" => video.height,
        "fps" => video.fps,
        "frames" => array![],
    };
    for frame in &video.frames {
        json["frames"].push(array![]).unwrap();
        for PixelUpdate { position, color } in frame {
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
    }
    json
}

fn write_file_u8(file: &PathBuf, value: &[u8]) {
    File::create(file).unwrap().write_all(value).unwrap();
}
