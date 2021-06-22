use crate::ofs::ofs::OverlayFs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub path: String,
    pub count: usize,
    pub wasted_size: u64,
}

pub struct Efficiency<'a> {
    ofs: &'a OverlayFs,
    duplicates: Vec<Info>
}

pub fn list_multiple_versions(ofs: &OverlayFs) -> Vec<Info> {
    let mut multiple_versions: Vec<Info> = vec![];

    for file in ofs.entries() {
        if !file.versions.is_empty() {
            let ver = file.versions.last().unwrap();
            let wasted = if ver.deleted {
                file.total_size
            } else {
                file.total_size - ver.size
            };
            let i = Info {
                path: format!("{}{}", file.path, file.name),
                count: file.versions.len() + 1,
                wasted_size: wasted
            };
            multiple_versions.push(i);
        }
    }
    multiple_versions.sort_by(|a, b| b.wasted_size.cmp(&a.wasted_size));
    multiple_versions
}

impl <'a> Efficiency<'a> {
    pub fn new(ofs: &OverlayFs) -> Efficiency {
        Efficiency {
            ofs,
            duplicates: list_multiple_versions(ofs),
        }
    }

    pub fn get_wasted_bytes(&self) -> u64 {
            self.duplicates.iter().map(|i| i.wasted_size)
            .sum()
    }

    pub fn get_duplicates(self) -> Vec<Info> {
        self.duplicates
    }
}

#[cfg(test)]
mod tests {
    use crate::analysis::efficiency::list_multiple_versions;
    use crate::ofs::directory::ODirectory;
    use crate::ofs::file_info::{FileInfo};
    use crate::ofs::ofs::OverlayFs;

    #[test]
    fn list_all_multi_version() {
        let ofs_json = r#"{"root":{"name":"/","size":0,"files":{"file1":{"name":"file1","size":400,"layer_id":"lay1","path":"","total_size":450,"versions":[{"deleted":false,"size":50,"layer_id":"lay2"}]}},"directories":{},"deleted":false},"layers":{}}"#;

        let ofs: OverlayFs = serde_json::from_str(&ofs_json).unwrap();

        let infos = list_multiple_versions(&ofs);
        assert_eq!(infos.len(), 1);
        assert_eq!(infos[0].path, "file1");
        assert_eq!(infos[0].wasted_size, 400);
    }
}
