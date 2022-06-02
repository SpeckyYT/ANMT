use std::ffi::OsStr;
use std::fs;
use std::iter::Iterator;
use std::path::{Path, PathBuf};

pub fn summon_folder(folder: &Path) {
    if !folder.exists() {
        let backwards = folder.join("..");
        if !backwards.exists() { summon_folder(&backwards) }
        fs::create_dir(folder).unwrap();
    }
}

pub fn find_files(folder: &Path, extensions: &[&str]) -> Vec<PathBuf> {
    return folder
    .read_dir()
    .unwrap()
    .filter(|entry| {
        let dir_entry = entry.as_ref().unwrap().to_owned();
        let path = dir_entry.path();
        if path.is_dir() { return false }
        let ext = path.extension().unwrap_or(OsStr::new("")).to_str().unwrap();
        return extensions.contains(&ext);
    })
    .map(|a| a.unwrap().path())
    .collect();
}
