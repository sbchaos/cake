use crate::image::Image;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use crate::image::manifest::Manifest;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub env: Vec<String>,
    pub user: Option<String>,
    pub cmd: Vec<String>,
    pub working_dir: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    pub created: String,
    pub author: Option<String>,
    pub created_by: String,
    pub empty_layer: Option<bool>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageConfig {
    pub config: Config,
    pub created: String,
    pub history: Vec<History>,
}

impl ImageConfig {
    pub fn for_path(image: &Image, manifest: &Manifest) -> serde_json::Result<ImageConfig> {
        let config_path = format!("{}/{}", image.image_id, manifest.config);
        let mut input = File::open(config_path).unwrap();

        let mut json = String::new();
        input.read_to_string(&mut json).unwrap();

        serde_json::from_str(&json)
    }

    pub fn layers(&self, manifest: &Manifest) {}
}
