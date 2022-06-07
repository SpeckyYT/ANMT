use std::path::PathBuf;
use std::process::{Command, Child};

pub fn extract_frames(frames_folder: &PathBuf, file_path: &PathBuf) -> Child {
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let current_folder = frames_folder.join(file_name);

    crate::util::summon_folder(&current_folder);

    return Command::new("ffmpeg")
    .args([
        "-i",
        file_path.to_str().unwrap(),
        current_folder.join(
            format!(
                "{}.%{}d.png",
                file_name,
                "6",
            ),
        ).to_str().unwrap(),
    ])
    .spawn()
    .expect("Failed to execute FFMPEG");
}
