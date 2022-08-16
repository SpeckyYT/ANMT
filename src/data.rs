use std::path::{Path, PathBuf};
use std::time::Duration;
use image::imageops::FilterType;

pub struct Video {
    pub path: PathBuf,
    pub frames: Vec<Vec<PixelUpdate>>,
    pub frame_count: usize,
    pub width: usize,
    pub height: usize,
    pub duration: f64,
    pub fps: f64,
    pub quiet: bool,
    pub time: Vec<(&'static str, Duration)>,
    pub skip_extract: bool,
    pub optimization: Optimization,
    pub filter: Filter,
    pub color_precision: u8,
    pub max_pixels: u32,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_pixel_update(&self, index: usize, width: usize) -> PixelUpdate {
        let (x, y) = crate::util::index_to_position(index, width);
        PixelUpdate {
            position: (x, y),
            color: *self,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct PixelUpdate {
    pub position: (usize, usize),
    pub color: Color,
}

impl Video {
    pub fn new(
        file_path: &Path,
        quiet: bool,
        skip_extract: bool,
        optimization: Optimization,
        filter: Filter,
        color_precision: u8,
        max_pixels: u32,
    ) -> Video {
        Video {
            path: file_path.to_path_buf(),
            frames: Vec::new(),
            frame_count: 0,
            width: 0,
            height: 0,
            duration: 0.0,
            fps: 0.0,
            quiet,
            time: Vec::default(),
            skip_extract,
            optimization,
            filter,
            color_precision,
            max_pixels,
        }
    }
    pub fn log(&self, message: String) {
        if !self.quiet {
            println!("[{}] {}", self.file_name(""), message);
        }
    }
    pub fn log_empty(&self) {
        self.log(String::new());
    }
    pub fn log_percent(&self, message: &str, current: usize, total: usize) {
        self.log(
            format!(
                "{}: {}/{} ({}%)",
                message,
                current,
                total,
                if total > 0 { 100 * current / total } else { 100 },
            )
        );
    }
    pub fn log_final(&self) {
        let required_frames = self.frames.iter().filter(|f| !f.is_empty()).count();

        self.log_empty();
        self.log(format!("({}/{}) total frames / required frames ({}%)", self.frame_count, required_frames, 100 * required_frames / self.frame_count));
        self.log(format!("{} frames per second", self.fps));
        self.log(format!("{} seconds duration", self.duration));
        self.log(format!("{} pixels ({}x{})", self.width as u16 * self.height as u16, self.width, self.height));
        self.log(format!("{} color change triggers", self.frames.iter().map(|f| f.len()).sum::<usize>()));
        self.log(format!("{} bits color precision", self.color_precision));
        self.log(format!("'{}' optimizazion", self.optimization.to_str()));
        self.log(format!("'{}' resizing filter", self.filter.to_str()));
        for (name, duration) in &self.time {
            self.log(format!("{}s {} time ({} average fps)", duration.as_secs_f64(), name, fps(*duration, self.frame_count)));
        }
    }
    pub fn file_name(&self, extension: &str) -> String {
        let file_name = self.path.file_stem().unwrap().to_str().unwrap();
        if !extension.is_empty() {
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

impl Optimization {
    pub fn to_str(&self) -> &str {
        match self {
            Optimization::None => "none",
            Optimization::Forward => "forward",
            Optimization::Backward => "backward",
            Optimization::Both => "both",
        }
    }
}

pub enum Filter {
    Nearest,
    Linear,
    Cubic,
    Gaussian,
    Lanczos3,
}

impl Filter {
    pub fn to_str(&self) -> &str {
        match self {
            Filter::Nearest => "nearest",
            Filter::Linear => "linear",
            Filter::Cubic => "cubic",
            Filter::Gaussian => "gaussian",
            Filter::Lanczos3 => "lanczos3",
        }
    }
    pub fn to_filter_type(&self) -> FilterType {
        match self {
            Filter::Nearest => FilterType::Nearest,
            Filter::Linear => FilterType::Triangle,
            Filter::Cubic => FilterType::CatmullRom,
            Filter::Gaussian => FilterType::Gaussian,
            Filter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}
