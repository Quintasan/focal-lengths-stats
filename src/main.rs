extern crate exif;
use exif::Tag;
use exif::In;
use walkdir::WalkDir;
use walkdir::DirEntry;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;
use std::time::Duration;
use indicatif::ProgressBar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut focals = Vec::new();
    let bar = ProgressBar::new_spinner();
    bar.set_message("Reading focal lengths from photos");
    bar.enable_steady_tick(Duration::from_millis(100));
    for file in list_files() {
        let focal = read_focal_length(file).unwrap_or("No focal length".to_string());
        focals.push(focal);
    };
    bar.finish();
    let counts = focals.clone().into_iter().counts();
    println!("{:#?}", counts);
    Ok(())
}

fn read_focal_length(dir_entry: DirEntry) -> Option<String> {
    let file = match std::fs::File::open(dir_entry.path()) {
        Ok(file) => file,
        _ => return None
    };
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => exif,
        _ => return None
    };
    match exif.get_field(Tag::FocalLength, In::PRIMARY) {
      Some(focal_length) => Some(focal_length.display_value().with_unit(()).to_string()),
      _ => None
    }
}

fn list_files() -> impl Iterator<Item = walkdir::DirEntry> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(arw|dng|nef)$").unwrap();
    }

    return WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some())
        .filter(|e| REGEX.is_match(e.path().extension().unwrap().to_str().unwrap()))
}
