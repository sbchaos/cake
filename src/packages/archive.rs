use super::super::ofs::ofs::OverlayFs;
use crate::ofs::file_info::FileInfo;
use crate::packages::deps::{Cache, Package, PackageManager};
use crate::packages::util::get_single_version_in_dir;
use log::trace;
use std::cmp::Ordering;

const EXTENSIONS: [&str; 5] = [".zip", ".tar", ".tar.gz", ".tar.bz", ".tar.xz"];

const TEMP_AS_CACHE: &str = "/tmp";

pub fn list_archives(ofs: &OverlayFs) -> Vec<&FileInfo> {
    let mut archives = vec![];

    for file in ofs.entries() {
        if EXTENSIONS.iter().any(|&ext| file.name.ends_with(ext)) {
            archives.push(file);
        }
    }
    archives
}

pub struct ArchiveManager<'a> {
    ofs: &'a OverlayFs,
    packages: Vec<Package>,
}

impl<'a> ArchiveManager<'a> {
    pub fn new(ofs: &'a OverlayFs) -> ArchiveManager<'a> {
        ArchiveManager {
            ofs,
            packages: get_packages(ofs),
        }
    }
    pub fn get_packages_from_docker() {
        // find / -type f '(' -name "*.zip" -o -name "*.tar*" ')' -printf '%P %k\n'
        // docker run --rm -it alpine-pkg /bin/find .
    }
}

fn get_packages(ofs: &OverlayFs) -> Vec<Package> {
    let mut packages = vec![];
    for f in list_archives(ofs) {
        let ver = f.versions.len() + 1;
        let pkg = Package {
            name: format!("{}{}", f.path, f.name),
            version: ver.to_string(),
            optional: false,
            size: f.total_size,
        };
        packages.push(pkg);
    }
    packages.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    packages
}

impl<'a> PackageManager for ArchiveManager<'a> {
    fn name(&self) -> &str {
        "Archives - zip/tar"
    }

    fn get_installed_packages(self) -> Vec<Package> {
        self.packages
    }

    fn get_cache(&self) -> Option<Cache> {
        if let Some(dir) = self.ofs.get_dir(TEMP_AS_CACHE) {
            return Some(Cache {
                path: TEMP_AS_CACHE.to_string(),
                size: dir.size(),
            });
        }
        None
    }

    fn get_wasted_bytes(&self) -> u64 {
        let cache_size = get_single_version_in_dir(self.ofs, TEMP_AS_CACHE);
        let pkg: u64 = self
            .packages
            .iter()
            .filter(|&a| a.version == "1")
            .map(|a| a.size)
            .sum();

        trace!("Single cache: {}, pkg: {}", cache_size, pkg);
        cache_size + pkg
    }
}
//  Add tokio task spawn to spawn the task and call them async

#[cfg(test)]
mod tests {
    use crate::ofs::ofs::OverlayFs;
    use crate::packages::archive::list_archives;
    use log::LevelFilter::Off;

    #[test]
    fn returns_a_list_of_archives() {
        let file_names = [
            "abc",
            "arch.zip",
            "kern.7z", // Not supported
            "files.rar",
            "hosts",
            "java.tar.gz",
            "python.tar.bz",
            "bin",
            "image.tar",
            "foo",
        ];
        let mut ofs = OverlayFs::new();
        for n in file_names.iter() {
            ofs.add_file(*n, 10, "abc");
        }

        let archives = list_archives(&ofs);
        println!("{:?}", archives);
        assert_eq!(archives.len(), 4);
        assert_eq!(archives.iter().any(|&f| f.name == "arch.zip"), true);
        assert_eq!(archives.iter().any(|&f| f.name == "java.tar.gz"), true);
        assert_eq!(archives.iter().any(|&f| f.name == "python.tar.bz"), true);
        assert_eq!(archives.iter().any(|&f| f.name == "image.tar"), true);
    }
}
