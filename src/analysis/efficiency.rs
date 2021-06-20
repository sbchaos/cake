use crate::ofs::file_info::FileInfo;
use crate::ofs::ofs::OverlayFs;

pub fn list_multiple_versions(ofs: &OverlayFs) -> Vec<&FileInfo> {
    let mut multiple_versions: Vec<&FileInfo> = vec![];

    for file in ofs.entries() {
        if !file.versions.is_empty() {
            multiple_versions.push(file);
        }
    }
    multiple_versions
}

pub fn get_wasted_bytes(ofs: &OverlayFs) -> u64 {
    list_multiple_versions(ofs)
        .iter()
        .map(|&fl| {
            let ver = fl.versions.last().unwrap();
            if ver.deleted {
                fl.total_size
            } else {
                fl.total_size - ver.size
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::analysis::efficiency::list_multiple_versions;
    use crate::ofs::directory::ODirectory;
    use crate::ofs::file_info::{FileInfo, MultiVersionInfo};
    use crate::ofs::ofs::OverlayFs;

    #[test]
    fn list_all_multi_version() {
        let ofs_json = "{\"root\":{\"name\":\"/\",\"size\": 8905,\"files\":{\"m1\": {\"Multi\":{\"name\":\"m1\", \"total_size\": 20, \"path\": \"/\", \"versions\": []}}, \"s1\": {\"Single\":{\"name\":\"s1\", \"size\":0, \"layer\":\"abc\", \"path\":\"/\"}}},\"directories\":{}, \"path\": \"/\", \"layer\": \"abc\"}}";
        let ofs: OverlayFs = serde_json::from_str(&ofs_json).unwrap();

        let mfs = list_multiple_versions(&ofs);
        assert_eq!(mfs.len(), 1);
        assert_eq!(mfs[0].name, "m1");
        assert_eq!(mfs[0].total_size, 20);
    }
}
