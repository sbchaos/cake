use crate::image::image_tar::get_file_from_layer;
use crate::ofs::ofs::OverlayFs;
use log::trace;

fn get_last_layer_with<'a>(ofs: &'a OverlayFs, path: &str) -> Option<&'a str> {
    if let Some(f) = ofs.get_file(path) {
        trace!("File found at path : {}", path);

        if f.versions.is_empty() {
            return Some(&f.layer_id);
        } else {
            for v in f.versions.iter().rev() {
                if !v.deleted {
                    return Some(&v.layer_id);
                }
            }
        }
    }
    None
}

pub fn get_file_from_image(ofs: &OverlayFs, path: &str) -> Option<String> {
    if let Some(layer_id) = get_last_layer_with(ofs, path) {
        trace!("Layer with file {}", layer_id);
        if let Some(l) = ofs.get_layer(layer_id) {
            let os_path_to_layer = &l.path;

            let mut path_str = path;
            if let Some(0) = path.find('/') {
                path_str = &path[1..];
            }
            return get_file_from_layer(&os_path_to_layer, path_str);
        }
    }
    None
}

pub fn get_single_version_in_dir(ofs: &OverlayFs, path: &str) -> u64 {
    let mut wasted: u64 = 0;
    if let Some(dir) = ofs.get_dir(path) {
        for e in dir.get_entries() {
            if e.versions.is_empty() {
                wasted += e.size;
            } else if let Some(last) = e.versions.last() {
                wasted += last.size
            }
        }
    }
    wasted
}
