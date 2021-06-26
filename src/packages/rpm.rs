use crate::docker;
use crate::ofs::ofs::OverlayFs;
use crate::packages::deps::{Cache, Package, PackageManager};
use crate::packages::util::get_single_version_in_dir;
use log::trace;
use std::cmp::Ordering;

#[allow(dead_code)]
const RPM_MACROS: &str = "/usr/lib/rpm/macros";

//RPM command to extract packages from the rpm database
const RPM_CMD: [&str; 6] = [
    "/usr/bin/rpm",
    "--nodigest",
    "--nosignature",
    "-qa",
    "--qf",
    "%{NAME}\t%{VERSION}-%{RELEASE}\t%{SIZE}\n",
];

// We can also use rpm to query the database, install rpm by installing
// brew install rpm
const CACHE_DIR: &str = "/var/cache/yum/";

pub struct RPMPackageManager<'a> {
    ofs: &'a OverlayFs,
    image: &'a str,
}

impl<'a> RPMPackageManager<'a> {
    pub fn new(ofs: &'a OverlayFs, image: &'a str) -> RPMPackageManager<'a> {
        RPMPackageManager { ofs, image }
    }

    #[allow(dead_code)]
    pub fn get_packages_from_docker() {
        // docker run --rm -it alpine-pkg /usr/bin/apt list --installed
    }
}

impl<'a> PackageManager for RPMPackageManager<'a> {
    fn name(&self) -> &str {
        "RPM - yum/dnf/micro-dnf"
    }

    fn get_installed_packages(self) -> Vec<Package> {
        let mut packages = vec![];
        if self.ofs.get_dir("/usr/lib/rpm/").is_none() {
            return packages;
        }

        // Extract the rpm db to folder and let local rpm read it.
        if let Ok(pkgs) = docker::run(self.image, RPM_CMD.to_vec()) {
            trace!("Received status file");
            for pkg in pkgs.lines() {
                let mut p = pkg.split('\t');
                let package = Package {
                    name: p.next().unwrap_or("").to_string(),
                    version: p.next().unwrap_or("").to_string(),
                    size: p.next().unwrap_or("0").parse::<u64>().unwrap_or(0),
                    optional: false,
                };
                packages.push(package);
            }
        }
        packages.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        packages
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
