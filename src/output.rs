use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Instant, Duration};
use std::fs::File;
use json::{object, array};
use rayon::prelude::*;

use crate::data::PixelUpdate;
use crate::Video;

impl Video {
    pub fn output_txt(self: &Video, output_folder: &Path) -> Duration {
        let time = Instant::now();
        let mut txt = String::new();
        txt.push_str(format!("{},{},{}\n", self.width, self.height, self.fps).as_str());

        txt.push_str(
            self.frames.iter().par_bridge().map(|frame| {
                frame.iter()
                .map(|PixelUpdate { position, color }|
                    format!(
                        "{},{},{},{},{}",
                        position.0,
                        position.1,
                        color.r,
                        color.g,
                        color.b
                    )
                )
                .collect::<Vec<String>>()
                .join(":")
            })
            .collect::<Vec<String>>()
            .join("\n")
            .as_str()
        );
        write_file_u8(
            &output_folder.join(self.file_name("txt")),
            txt.as_bytes(),
        );

        time.duration_since(Instant::now())
    }
    pub fn output_anmt(self: &Video, output_folder: &Path) -> Duration {
        let time = Instant::now();
        let mut anmt = vec![
            self.width as u8,
            self.height as u8,
            self.fps.floor().clamp(0.0, 255.0) as u8,
            (self.fps.fract() * 256.0) as u8,
        ];
    
        for frame in &self.frames {
            anmt.push(0);
            for PixelUpdate { position, color } in frame {
                anmt.push(position.0 as u8 + 1);
                anmt.push(position.1 as u8 + 1);
                anmt.push(color.r);
                anmt.push(color.g);
                anmt.push(color.b);
            }
        }
        write_file_u8(
            &output_folder.join(self.file_name("anmt")),
            &anmt,
        );

        time.duration_since(Instant::now())
    }
    pub fn output_json(self: &Video, output_folder: &Path) -> Duration {
        let time = Instant::now();
        let mut json = object!{
            "width" => self.width,
            "height" => self.height,
            "fps" => self.fps,
        };
        json.insert(
            "frames",
            self.frames.par_iter().map(|frame| {
                frame.par_iter().map(|PixelUpdate { position, color }| {
                    array![
                        position.0,
                        position.1,
                        color.r,
                        color.g,
                        color.b,
                    ]
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
        ).unwrap();
        write_file_u8(
            &output_folder.join(self.file_name("json")),
            json.to_string().as_bytes(),
        );

        time.duration_since(Instant::now())
    }
}

fn write_file_u8(file: &PathBuf, value: &[u8]) {
    File::create(file).unwrap().write_all(value).unwrap();
}