use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Layer {
    pub layer_id: String,
    pub path: String,
}

impl Layer {
    pub fn new(layer_id: String, path: String) -> Layer {
        Layer { layer_id, path }
    }
}
