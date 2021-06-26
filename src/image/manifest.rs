use crate::image::Image;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Error;
use std::io::Read;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    pub config: String,
    pub repo_tags: Option<Vec<String>>,
    pub layers: Vec<String>,
}

impl Manifest {
    pub fn from(string: &str) -> Manifest {
        let manifests: Vec<Manifest> = serde_json::from_str(string).unwrap();
        manifests[0].to_owned()
    }

    pub fn for_image_path(image: &Image) -> Result<Manifest, Error> {
        let manifest_path = format!("{}/manifest.json", image.image_id);
        let mut input = File::open(manifest_path)?;

        let mut json = String::new();
        input.read_to_string(&mut json)?;

        Ok(Manifest::from(&json))
    }
}

#[cfg(test)]
mod tests {
    use crate::image::manifest::Manifest;

    #[test]
    fn deserialize_manifest_string() {
        let str_manifest = "[{\"Config\":\"6dbb9cc54074106d46d4ccb330f2a40a682d49dda5f4844962b7dce9fe44aaec.json\",\"RepoTags\":null,\"Layers\":[\"2309bd4f07083b22f40dfc9a1241015274e232cb15afccc17dac0ece450d70e7/layer.tar\"]}]".to_string();

        let manifest = Manifest::from(&str_manifest);
        assert_eq!(
            manifest.config,
            "6dbb9cc54074106d46d4ccb330f2a40a682d49dda5f4844962b7dce9fe44aaec.json"
        );
    }

    #[test]
    fn read_from_path() {
        let path = "test_files";
        let manifest = Manifest::for_image_path(path).unwrap();
        assert_eq!(
            manifest.config,
            "6dbb9cc54074106d46d4ccb330f2a40a682d49dda5f4844962b7dce9fe44aaec.json"
        );
    }
}
