pub mod image_tar;
pub mod inspect;
pub mod manifest;
pub mod config;

use super::docker;
use crate::image::Source::{Dir, Report, Tar, Tree};
use log::trace;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    Report,
    Tree,
    Dir,
    Tar,
    Docker,
    None,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub image_id: String,
    pub source: Source,
}

fn check_existence(str: &str) -> Source {
    trace!("checking existence for {}", str);
    let report = format!("{}_report.json", str);
    if Path::new(&report).exists() {
        return Report;
    }

    let tree = format!("{}.json", str);
    if Path::new(&tree).exists() {
        return Tree;
    }

    let dir_name = format!("{}/", str);
    if Path::new(&dir_name).exists() {
        return Dir;
    }

    let tar_name = format!("{}.tar", str);
    if Path::new(&tar_name).exists() {
        return Tar;
    }

    if let Ok(data) = docker::inspect(str) {
        if data.len() > 5 {
            trace!("Got response from inspect {:#?}", data);
            return Source::Docker;
        }
    }

    Source::None
}

impl Image {
    pub fn from(string: &str) -> Image {
        match check_existence(string) {
            Source::None => {}
            s => {
                return Image {
                    name: string.to_string(),
                    image_id: string.to_string(),
                    source: s,
                };
            }
        };

        let id = docker::image_id(string);
        match check_existence(&id) {
            Source::None => {
                panic!("Not able to find the image: {}", string);
            }
            s => Image {
                name: string.to_string(),
                image_id: id,
                source: s,
            },
        }
    }

    pub fn report_path(&self) -> String {
        format!("{}_report.json", self.image_id)
    }
    pub fn tree_path(&self) -> String {
        format!("{}.json", self.image_id)
    }
    pub fn dir_path(&self) -> String {
        format!("{}/", self.image_id)
    }
    pub fn tar_path(&self) -> String {
        format!("{}.tar", self.image_id)
    }
}
