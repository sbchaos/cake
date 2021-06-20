// Possible things, discard files less than 1 kb
// Keep dir info in memory, and only create a dir when any file > 1kb

use super::directory::ODirectory;
use crate::ofs::file_info::FileInfo;
use crate::ofs::layer::Layer;
use crate::ofs::utils::split_last_entry;
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};

#[derive(Serialize, Deserialize)]
pub struct OverlayFs {
    root: ODirectory,
    layers: HashMap<String, Layer>,
}

pub const WHITEOUT: &str = ".wh.";

impl OverlayFs {
    pub fn new() -> OverlayFs {
        OverlayFs {
            root: ODirectory::new("/"),
            layers: HashMap::new(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.insert(layer.layer_id.clone(), layer);
    }

    pub fn get_layer(&self, layer_id: &str) -> Option<&Layer> {
        self.layers.get(layer_id)
    }

    pub fn size(&self) -> u64 {
        self.root.size()
    }

    pub fn add_file(&mut self, path: &str, size: u64, layer_id: &str) {
        let (p, name) = split_last_entry(path);
        if name.starts_with(WHITEOUT) {
            let new_path = format!("{}{}", p, name.strip_prefix(WHITEOUT).unwrap());
            trace!("Received deleted file {}, removing {}", path, new_path);
            self.root.mark_for_delete(&new_path, layer_id);
        } else {
            self.root.add_file(path, size, layer_id);
        }
    }

    pub fn add_dir(&mut self, path: &str) {
        self.root.add_dir_path(path);
    }

    pub fn show_as_tree(&self) {
        println!("/");
        self.root.show_dir(0);
    }

    pub fn update_sizes(&mut self) {
        self.root.update_sizes();
    }

    pub fn save_tree_to_json(&self, image: &str) {
        let result = serde_json::to_string(&self).unwrap();
        let mut file = File::create(format!("{}.json", image)).unwrap();
        file.write_all(result.as_ref()).unwrap();
    }

    pub fn create_fs_from_json(image: &str) -> OverlayFs {
        let file = File::open(format!("{}.json", image)).unwrap();
        let mut buf_reader = BufReader::new(file);

        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        serde_json::from_str(&contents).unwrap()
    }

    pub fn entries(&self) -> Vec<&FileInfo> {
        self.root.get_entries()
    }

    pub fn get_dir(&self, path: &str) -> Option<&ODirectory> {
        self.root.get_dir(path)
    }

    pub fn get_file(&self, path: &str) -> Option<&FileInfo> {
        self.root.get_file(path)
    }
}
