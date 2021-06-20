use crate::ofs::utils::size_human;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionFile {
    pub deleted: bool,
    pub size: u64,
    pub layer_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub layer_id: String,
    pub path: String,
    pub total_size: u64,
    pub versions: Vec<VersionFile>,
}

impl FileInfo {
    pub fn new(name: &str, size: u64, layer_id: &str, path: &str) -> FileInfo {
        FileInfo {
            name: name.to_string(),
            size,
            layer_id: layer_id.to_string(),
            path: path.to_string(),
            total_size: size,
            versions: vec![],
        }
    }

    pub fn delete(&mut self, layer_id: &str) {
        let delete_file = VersionFile {
            deleted: true,
            size: 0,
            layer_id: layer_id.to_string(),
        };

        self.versions.push(delete_file);
    }

    pub fn add_version(&mut self, size: u64, layer_id: &str) {
        let version = VersionFile {
            deleted: false,
            size,
            layer_id: layer_id.to_string(),
        };

        self.total_size += size;
        self.versions.push(version);
    }

    pub fn show_file(&self) -> String {
        format!("{} - {}", self.name, size_human(self.total_size))
    }
}

#[cfg(test)]
mod tests {
    use super::{FileInfo, VersionFile};

    #[test]
    fn adds_version_for_a_file() {
        let mut file = FileInfo {
            name: "name".to_string(),
            size: 10,
            layer_id: "".to_string(),
            path: "".to_string(),
            total_size: 10,
            versions: vec![],
        };

        file.add_version(20, "lay2");
        assert_eq!(file.versions.len(), 1);
        assert_eq!(file.total_size, 30);
    }

    #[test]
    fn marks_file_for_delete() {
        let mut file = FileInfo {
            name: "name".to_string(),
            size: 10,
            layer_id: "".to_string(),
            path: "".to_string(),
            total_size: 10,
            versions: vec![],
        };

        file.delete("lay2");
        assert_eq!(file.versions.len(), 1);
        assert_eq!(file.total_size, 10);
    }

    #[test]
    fn gets_the_size_of_files() {
        let mut multi = FileInfo {
            name: "/".to_string(),
            size: 200,
            total_size: 200,
            path: "/".to_string(),
            versions: vec![],
            layer_id: "".to_string(),
        };
        multi.add_version(200, "lay2");
        multi.add_version(50, "lay3");

        assert_eq!(multi.total_size, 450);
    }

    #[test]
    fn serialises_multi_file_info() {
        let multi = FileInfo {
            name: "file1".to_string(),
            size: 50,
            total_size: 450,
            path: "/".to_string(),
            layer_id: "lay1".to_string(),
            versions: vec![VersionFile {
                deleted: false,
                size: 400,
                layer_id: "lay2".to_string(),
            }],
        };

        let result = serde_json::to_string(&multi).unwrap();
        assert_eq!(
            result,
            r#"{"name":"file1","size":50,"layer":"lay1","path":"/","total_size":450,"versions":[{"deleted":false,"size":400,"layer":"lay2"}]}"#
        );
    }
}
