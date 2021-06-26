use log::trace;
use std::fs::File;
use tar::{Archive, EntryType};

use super::super::ofs::ofs::OverlayFs;
use crate::image::Image;
use std::io::Read;

pub fn extract_image_tar(image: &Image) {
    let file = File::open(image.tar_path()).unwrap();
    let mut archive = Archive::new(file);

    archive.unpack(image.dir_path()).unwrap();
}

pub fn read_tar_layer(ofs: &mut OverlayFs, path: &str, layer_id: &str) {
    let file = File::open(path).unwrap();
    let mut a = Archive::new(file);

    for file in a.entries().unwrap() {
        let entry = file.unwrap();
        let header = entry.header();
        let cow = header.path().unwrap();
        let path_str = cow.to_str().unwrap();

        match header.entry_type() {
            EntryType::Directory => ofs.add_dir(path_str),

            EntryType::Regular => ofs.add_file(path_str, header.size().unwrap(), layer_id),
            _ => {}
        }
    }
}

pub fn get_file_from_layer(os_path: &str, file_path: &str) -> Option<String> {
    trace!("Searching file: {} in layer: {}", file_path, os_path);
    let file = File::open(os_path).unwrap();
    let mut a = Archive::new(file);

    for f in a.entries().unwrap() {
        let mut entry = f.unwrap();

        let cow = entry.path().unwrap();
        let path_str = cow.to_str().unwrap();
        if path_str == file_path {
            let mut s = String::new();
            entry.read_to_string(&mut s).unwrap();
            return Some(s);
        }
    }
    None
}
