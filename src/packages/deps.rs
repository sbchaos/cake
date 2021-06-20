use crate::ofs::utils::size_human;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub size: u64,
    pub optional: bool,
}

impl Package {
    pub fn show_info(&self) {
        println!(
            "{:>10}\t{:<20}\t{}",
            size_human(self.size),
            self.version,
            self.name
        )
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.size.cmp(&other.size) {
            //  By default sort in rev
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Greater => Some(Ordering::Less),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub path: String,
    pub size: u64,
}

pub trait PackageManager {
    fn name(&self) -> &str;
    fn get_installed_packages(self) -> Vec<Package>;
    fn get_cache(&self) -> Option<Cache>;
    fn get_wasted_bytes(&self) -> u64;
}
