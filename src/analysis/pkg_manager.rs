use crate::packages::deps::{Package, Cache};
use crate::ofs::utils::size_human;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Manager {
    pub name: String,
    pub packages: Vec<Package>,
    pub cache: Option<Cache>,
    pub waste_size: u64,
}

impl Manager {
    pub fn show_report(&self) {
        let total_pkg_size = self.packages.iter().map(|p| p.size).sum();
        let optional_pkg_size = self
            .packages
            .iter()
            .filter(|p| p.optional)
            .map(|p| p.size)
            .sum();

        if total_pkg_size == 0 {
            return;
        }

        println!("{}", &self.name);
        println!("All packages:  {:>10}", size_human(total_pkg_size));
        if optional_pkg_size > 0 {
            println!("Optional pkgs: {:>10}", size_human(optional_pkg_size));
        }
        if let Some(che) = &self.cache {
            println!("Cache:         {:>10} ({})", size_human(che.size), che.path);
        }
        println!();
    }

    pub fn show_packages(&self) {
        for pkg in self.packages.iter() {
            pkg.show_info();
        }
    }
}
