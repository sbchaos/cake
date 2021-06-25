use crate::ofs::file_info::FileInfo;
use crate::ofs::utils::{
    get_leading_entry, get_remaining, size_human, split_last_entry, BRANCH_SPACE, MIDDLE_ITEM,
};
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ODirectory {
    name: String,
    size: u64,
    files: HashMap<String, FileInfo>,
    directories: HashMap<String, ODirectory>,
}

impl ODirectory {
    pub fn new(name: &str) -> ODirectory {
        ODirectory {
            name: String::from(name),
            size: 0,
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn get_dir_at_path(&mut self, path: &str) -> Option<&mut ODirectory> {
        if path == "/" || path.is_empty() {
            Some(self)
        } else if self.directories.contains_key(get_leading_entry(path)) {
            let odir = self.directories.get_mut(get_leading_entry(path)).unwrap();
            odir.get_dir_at_path(get_remaining(path))
        } else {
            None
        }
    }

    pub fn add_dir(&mut self, d: ODirectory) {
        self.directories.insert(d.name.clone(), d);
    }

    pub fn add_dir_path(&mut self, path: &str) {
        if path.is_empty() || path == "/" {
        } else if self.directories.contains_key(get_leading_entry(path)) {
            let dir = self.directories.get_mut(get_leading_entry(path)).unwrap();
            dir.add_dir_path(get_remaining(path))
        } else {
            let fragment = get_leading_entry(path);
            let mut d = ODirectory::new(fragment);
            d.add_dir_path(get_remaining(path));
            self.add_dir(d);
        }
    }

    pub fn add_file(&mut self, path: &str, size: u64, layer_id: &str) {
        let (p, name) = split_last_entry(path);

        if let Some(d) = self.get_dir_at_path(p) {
            return match d.files.get_mut(name) {
                Some(f) => {
                    f.add_version(size, layer_id);
                    // d.add_with_existing_file(name, file);
                }
                None => {
                    let file = FileInfo::new(name, size, layer_id, p);
                    d.files.insert(name.to_string(), file);
                }
            };
        }

        self.add_dir_path(p);
        self.add_file(path, size, layer_id);
    }

    fn mark_children_delete(&mut self, layer_id: &str) {
        for (_name, f) in self.files.iter_mut() {
            f.delete(layer_id)
        }

        for (_n, d) in self.directories.iter_mut() {
            d.mark_children_delete(layer_id);
        }
    }

    pub fn mark_for_delete(&mut self, path: &str, layer_id: &str) {
        let (p, name) = split_last_entry(path);

        if let Some(d) = self.get_dir_at_path(p) {
            if let Some(dir) = d.directories.get_mut(name) {
                trace!("Marking children for delete at {}", dir.name);
                dir.mark_children_delete(layer_id);
            } else if let Some(f) = d.files.get_mut(name) {
                f.delete(layer_id);
            }
        }
    }

    pub fn show_dir(&self, levels: usize) {
        let padding = BRANCH_SPACE.repeat(levels);
        let marker = MIDDLE_ITEM;
        for (name, dir) in self.directories.iter() {
            println!("{}{} {} - {}", padding, marker, name, size_human(dir.size));
            dir.show_dir(levels + 1);
        }
        for file in self.files.values() {
            println!("{}{} {}", padding, marker, file.show_file());
        }
    }

    pub fn update_sizes(&mut self) -> u64 {
        let mut size = 0;

        for f in self.files.values() {
            size += f.total_size
        }

        for d in self.directories.values_mut() {
            size += d.update_sizes();
        }

        self.size = size;
        size
    }

    pub fn get_entries(&self) -> Vec<&FileInfo> {
        let mut files = vec![];
        files.extend(self.files.values());

        for d in self.directories.values() {
            files.extend(&d.get_entries())
        }

        files
    }

    pub fn get_dir(&self, path: &str) -> Option<&ODirectory> {
        if path == "/" || path.is_empty() {
            Some(self)
        } else if self.directories.contains_key(get_leading_entry(path)) {
            let odir = self.directories.get(get_leading_entry(path)).unwrap();
            odir.get_dir(get_remaining(path))
        } else {
            None
        }
    }

    pub fn get_file(&self, path: &str) -> Option<&FileInfo> {
        let (p, name) = split_last_entry(path);

        if let Some(d) = self.get_dir(p) {
            d.files.get(name)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ofs::directory::ODirectory;
    use crate::ofs::file_info::FileInfo;
    use std::collections::HashMap;

    #[test]
    fn gets_dir_at_path() {
        let mut root = ODirectory::new("/");

        let path = "/usr/local/bin/";
        root.add_dir_path(path);

        let dir = root.get_dir_at_path("/usr/local/bin").unwrap();
        assert_eq!(dir.name, "bin")
    }

    #[test]
    fn gets_file_at_path() {
        let mut root = ODirectory::new("/");
        let path = "/usr/local/bin/git";
        root.add_file(path, 200, "abc");

        let file = root.get_file("/usr/local/bin/git").unwrap();
        assert_eq!(file.name, "git");
        assert_eq!(file.size, 200);
    }

    #[test]
    fn add_file_at_path() {
        let mut root = ODirectory::new("/");
        root.add_dir_path("/usr/");
        root.add_file("/usr/local/bin/git", 30000, "test");

        let dir = root.get_dir_at_path("/usr/local/bin/").unwrap();
        let file = dir.files.get("git").unwrap();
        assert_eq!(file.name, "git");
        assert_eq!(file.size, 30000);
        assert_eq!(file.total_size, 30000);
    }

    #[test]
    fn adds_new_version_to_a_path() {
        let mut root = ODirectory::new("/");
        root.add_dir_path("/usr/");
        root.add_file("/usr/local/bin/git", 30000, "test");

        root.add_file("/usr/local/bin/git", 40000, "test2");

        let bin = root.get_dir_at_path("/usr/local/bin").unwrap();
        let f = bin.files.get("git").unwrap();

        assert_eq!(f.name, "git");
        assert_eq!(f.total_size, 70000);
    }

    #[test]
    fn updates_size_of_dirs() {
        let mut root = ODirectory::new("/");
        root.add_dir_path("/usr/");
        root.add_file("/usr/local/bin/git", 30000, "test");
        root.add_file("/usr/local/bin/git", 40000, "test2");
        root.add_file("/usr/local/brew", 1200, "test3");

        root.update_sizes();
        assert_eq!(root.size, 71200);
        assert_eq!(root.get_dir_at_path("/usr").unwrap().size, 71200);
        assert_eq!(root.get_dir_at_path("/usr/local").unwrap().size, 71200);
        assert_eq!(root.get_dir_at_path("/usr/local/bin").unwrap().size, 70000);
    }
}
