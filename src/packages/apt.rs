use crate::ofs::ofs::OverlayFs;
use crate::packages::deps::{Cache, Package, PackageManager};
use crate::packages::util::{get_file_from_image, get_single_version_in_dir};
use log::trace;
use std::cmp::Ordering;

// Cache is at /var/cache/apt/archives but official ubuntu images clean archives
// https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#run
const CACHE_DIR: &str = "/var/cache/apt/";
const LISTS_DIR: &str = "/var/lib/apt/lists/";
const STATUS_FILE: &str = "/var/lib/dpkg/status";

pub struct DebianPackageManager<'a> {
    ofs: &'a OverlayFs,
    packages: Vec<Package>,
}

impl<'a> DebianPackageManager<'a> {
    pub fn new(ofs: &'a OverlayFs) -> DebianPackageManager<'a> {
        DebianPackageManager {
            ofs,
            packages: get_packages(ofs),
        }
    }

    pub fn get_packages_from_docker() {
        // docker run --rm -it alpine-pkg /usr/bin/apt list --installed
    }
}

fn get_packages(ofs: &OverlayFs) -> Vec<Package> {
    if let Some(content) = get_file_from_image(ofs, STATUS_FILE) {
        trace!("Received status file");
        let mut pkgs = parse_status_file(&content);
        pkgs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        return pkgs;
    }
    vec![]
}

fn parse_status_file(status: &str) -> Vec<Package> {
    let mut packages = vec![];

    let mut lines = status.lines();
    while let Some(l) = lines.next() {
        let mut name = l.strip_prefix("Package: ").unwrap_or("");
        let mut version = "";
        let mut size: u64 = 0;
        let mut optional = false;

        loop {
            let d = lines.next();
            match d {
                None => break,
                Some("") => break,
                Some(p) => {
                    if p.starts_with("Package: ") {
                        name = p.strip_prefix("Package: ").unwrap();
                    } else if p.starts_with("Version: ") {
                        version = p.strip_prefix("Version: ").unwrap();
                    } else if p.starts_with("Installed-Size: ") {
                        let size_in_kb = p
                            .strip_prefix("Installed-Size: ")
                            .unwrap()
                            .parse::<u64>()
                            .unwrap_or(0);
                        size = size_in_kb * 1024;
                    } else if p.starts_with("Priority: optional") {
                        optional = true;
                    }
                }
            }
        }
        let package = Package {
            name: name.to_string(),
            version: version.to_string(),
            optional,
            size,
        };
        packages.push(package)
    }
    packages
}

impl<'a> PackageManager for DebianPackageManager<'a> {
    fn name(&self) -> &str {
        "APT - apt-get/aptitude"
    }

    fn get_installed_packages(self) -> Vec<Package> {
        self.packages
    }

    fn get_cache(&self) -> Option<Cache> {
        if let Some(dir) = self.ofs.get_dir(CACHE_DIR) {
            if dir.size() > 1024 {
                return Some(Cache {
                    path: CACHE_DIR.to_string(),
                    size: dir.size(),
                });
            }
            if let Some(lst) = self.ofs.get_dir(LISTS_DIR) {
                return Some(Cache {
                    path: LISTS_DIR.to_string(),
                    size: lst.size(),
                });
            }
        }
        None
    }

    fn get_wasted_bytes(&self) -> u64 {
        let cache_size = get_single_version_in_dir(self.ofs, CACHE_DIR);
        let optional_pkg_size: u64 = self
            .packages
            .iter()
            .filter(|p| p.optional)
            .map(|p| p.size)
            .sum();
        cache_size + optional_pkg_size
    }
}

#[cfg(test)]
mod tests {
    use crate::packages::apt::parse_status_file;

    #[test]
    fn should_be_able_to_parse_packages() {
        let installed_string = r#"Package: less
Status: install ok installed
Priority: important
Section: text
Installed-Size: 296
Maintainer: Milan Kupcevic <milan@debian.org>
Architecture: amd64
Multi-Arch: foreign
Version: 551-2
Depends: libc6 (>= 2.14), libtinfo6 (>= 6)
Description: pager program similar to more
 This package provides "less", a file pager (that is, a memory-efficient
 utility for displaying text one screenful at a time). Less has many
 more features than the basic pager "more". As part of the GNU project,
 it is widely regarded as the standard pager on UNIX-derived systems.
 .
 Also provided are "lessecho", a simple utility for ensuring arguments
 with spaces are correctly quoted; "lesskey", a tool for modifying the
 standard (vi-like) keybindings; and "lesspipe", a filter for specific
 types of input, such as .doc or .txt.gz files.
Homepage: http://www.greenwoodsoftware.com/less/

Package: libacl1
Status: install ok installed
Priority: optional
Section: libs
Installed-Size: 71
Maintainer: Guillem Jover <guillem@debian.org>
Architecture: amd64
Multi-Arch: same
Source: acl
Version: 2.2.53-10
Depends: libc6 (>= 2.14)"#;

        let installed = parse_status_file(installed_string);

        assert_eq!(installed.len(), 2);
        assert_eq!(installed[0].name, "less");
        assert_eq!(installed[0].version, "551-2");
        assert_eq!(installed[0].optional, false);
        assert_eq!(installed[0].size, 296);

        assert_eq!(installed[1].name, "libacl1");
        assert_eq!(installed[1].version, "2.2.53-10");
        assert_eq!(installed[1].optional, true);
        assert_eq!(installed[1].size, 71);
    }
}
