use std::path::PathBuf;
use std::time::Duration;
use image::imageops::FilterType;

pub struct Video {
    pub path: PathBuf,
    pub frames: Vec<Vec<PixelUpdate>>,
    pub width: u8, // atm u8 are enough
    pub height: u8,
    pub duration: f64,
    pub fps: f64,
    pub quiet: bool,
    pub extract_time: Duration,
    pub process_time: Duration,
    pub output_time: Duration,
    pub skip_extract: bool,
    pub optimization: Optimization,
    pub filter: FilterType,
}

pub struct PixelUpdate {
    pub position: (u8, u8),
    pub color: [u8; 4],
}

impl Video {
    pub fn new(
        file_path: &PathBuf,
        quiet: bool,
        skip_extract: bool,
        optimization: Optimization,
        filter: FilterType,
    ) -> Video {
        Video {
            path: file_path.to_path_buf(),
            frames: Vec::new(),
            width: 0,
            height: 0,
            duration: 0.0,
            fps: 0.0,
            quiet: quiet,
            extract_time: Duration::default(),
            process_time: Duration::default(),
            output_time: Duration::default(),
            skip_extract: skip_extract,
            optimization: optimization,
            filter: filter,
        }
    }
    pub fn log(&self, message: String) {
        if !self.quiet {
            println!("[{}] {}", self.file_name(""), message);
        }
    }
    pub fn log_percent(&self, message: &str, current: usize, total: usize) {
        self.log(
            format!(
                "{}: {}/{} ({}%)",
                message,
                current,
                total,
                100 * current / total,
            )
        );
    }
    pub fn log_final(&self) {
        self.log(format!("{} frames", self.frames.len()));
        self.log(format!("{} frames per second", self.fps));
        self.log(format!("{} seconds duration", self.duration));
        self.log(format!("{} pixels ({}x{})", self.width as u16 * self.height as u16, self.width, self.height));
        self.log(format!("{} color change triggers", self.frames.iter().map(|f| f.len()).sum::<usize>()));
        self.log(format!("{}s extract time ({} average fps)", self.extract_time.as_secs_f64(), fps(self.extract_time, self.frames.len())));
        self.log(format!("{}s process time ({} average fps)", self.process_time.as_secs_f64(), fps(self.process_time, self.frames.len())));
        self.log(format!("{}s output time ({} average fps)", self.output_time.as_secs_f64(), fps(self.output_time, self.frames.len())));
    }
    pub fn file_name(&self, extension: &str) -> String {
        let file_name = self.path.file_stem().unwrap().to_str().unwrap();
        if extension.len() > 0 {
            format!("{}.{}", file_name, extension)
        } else {
            file_name.to_string()
        }
    }    
}

fn fps(time: Duration, frames: usize) -> f64 {
    if frames == 0 {
        0.0
    } else {
        frames as f64 / time.as_secs_f64()
    }
}

pub enum Optimization {
    None,
    Forward,
    Backward,
    Both,       // will actually use more objects than Forward or Backward
}
