use crate::ofs::ofs::OverlayFs;
use crate::packages::deps::{Cache, Package, PackageManager};
use crate::packages::util::{get_file_from_image, get_single_version_in_dir};
use log::trace;
use std::cmp::Ordering;
use std::option::Option::Some;

const CACHE_DIR: &str = "/var/cache/apk/";
const INSTALLED_FILE: &str = "/lib/apk/db/installed";

pub struct AlpinePackageManager<'a> {
    ofs: &'a OverlayFs,
}

impl<'a> AlpinePackageManager<'a> {
    pub fn new(ofs: &'a OverlayFs) -> AlpinePackageManager<'a> {
        AlpinePackageManager { ofs }
    }

    #[allow(dead_code)]
    pub fn get_packages_from_docker() {
        // docker run --rm -it alpine-pkg /sbin/apk -v info
    }
}

fn parse_installed_status_file(status: &str) -> Vec<Package> {
    let mut packages = vec![];

    let mut lines = status.lines();
    while let Some(_l) = lines.next() {
        // _l is checksum
        // We are skipping first thing after empty line C:Q10Nu5
        let mut name = "";
        let mut version = "";
        let mut size: u64 = 0;

        loop {
            let d = lines.next();
            match d {
                None => break,
                Some("") => break,
                Some(p) => {
                    if p.starts_with("P:") {
                        name = p.strip_prefix("P:").unwrap();
                    } else if p.starts_with("V:") {
                        version = p.strip_prefix("V:").unwrap();
                    } else if p.starts_with("I:") {
                        size = p.strip_prefix("I:").unwrap().parse::<u64>().unwrap_or(0);
                    }
                }
            }
        }
        let package = Package {
            name: name.to_string(),
            version: version.to_string(),
            optional: false,
            size,
        };
        packages.push(package)
    }
    packages
}

impl<'a> PackageManager for AlpinePackageManager<'a> {
    fn name(&self) -> &str {
        "alpine - apk"
    }

    fn get_installed_packages(self) -> Vec<Package> {
        if let Some(content) = get_file_from_image(&self.ofs, INSTALLED_FILE) {
            trace!("Received status file");
            let mut pkgs = parse_installed_status_file(&content);
            pkgs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            return pkgs;
        }
        vec![]
    }

    fn get_cache(&self) -> Option<Cache> {
        if let Some(dir) = self.ofs.get_dir(CACHE_DIR) {
            return Some(Cache {
                path: CACHE_DIR.to_string(),
                size: dir.size(),
            });
        }
        None
    }

    fn get_wasted_bytes(&self) -> u64 {
        get_single_version_in_dir(self.ofs, CACHE_DIR)
    }
}

#[cfg(test)]
mod tests {
    use crate::packages::alpine::parse_installed_status_file;

    #[test]
    fn should_be_able_to_parse_packages() {
        let installed_string = r#"C:Q10Nu5eN4wrh8WXuJA2JaSwqOlJyE=
P:musl
V:1.2.2-r0
A:x86_64
S:382732
I:622592
T:the musl c library (libc) implementation
U:https://musl.libc.org/
L:MIT
o:musl
m:Timo Teräs <timo.teras@iki.fi>
t:1610703982
c:c6c2cd54a0db8503a7e0238f388e1daff35d5d4d
p:so:libc.musl-x86_64.so.1=1
F:lib
R:ld-musl-x86_64.so.1
a:0:0:755
Z:Q1+iSDev5zZq96D14Qgc18qcKJ+Qk=
R:libc.musl-x86_64.so.1
a:0:0:777
Z:Q17yJ3JFNypA4mxhJJr0ou6CzsJVI=

C:Q1qwlR6vNeSFcNQWzpcifus9YorNk=
P:busybox
V:1.32.1-r6
A:x86_64
S:497774
I:946176
T:Size optimized toolbox of many common UNIX utilities
U:https://busybox.net/
L:GPL-2.0-only
o:busybox
m:Natanael Copa <ncopa@alpinelinux.org>
t:1618390590
c:8f37ff27685a4e44ede31c6738661032f6656668
D:so:libc.musl-x86_64.so.1
p:/bin/sh cmd:busybox cmd:sh
r:busybox-initscripts
F:bin
R:busybox
a:0:0:755
Z:Q1ccKCuw60J+4z1H9b9mYgXQ+GonI=
R:sh"#;

        let installed = parse_installed_status_file(installed_string);

        assert_eq!(installed.len(), 2);
        assert_eq!(installed[0].name, "musl");
        assert_eq!(installed[0].version, "1.2.2-r0");
        assert_eq!(installed[0].size, 622592);

        assert_eq!(installed[1].name, "busybox");
        assert_eq!(installed[1].version, "1.32.1-r6");
        assert_eq!(installed[1].size, 946176);
    }
}
