use super::pkg_manager::Manager;
use crate::analysis::efficiency::{Efficiency, Info};
use crate::image::Image;
use crate::ofs::ofs::OverlayFs;
use crate::ofs::utils::size_human;
use crate::style::{bold, green, red, yellow};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};

#[derive(Serialize, Deserialize)]
pub struct AnalysisReport {
    pub score: u64,
    pub wasted_space: u64,
    pub total_space: u64,
    pub ofs: OverlayFs,
    pub image: Image,

    pub managers: Vec<Manager>,
    pub dup_files: Vec<Info>,
}

impl AnalysisReport {
    pub fn create_analysis_report(
        ofs: OverlayFs,
        image: Image,
        managers: Vec<Manager>,
    ) -> AnalysisReport {
        let eff = Efficiency::new(&ofs);
        let waste = eff.get_wasted_bytes();
        let pkg_waste: u64 = managers.iter().map(|m| m.waste_size).sum();
        let size = ofs.size();

        let score = ((size - (waste + pkg_waste)) * 100) / size;
        let dup_files = eff.get_duplicates();

        AnalysisReport {
            score,
            wasted_space: waste + pkg_waste,
            total_space: size,
            ofs,
            image,
            managers,
            dup_files,
        }
    }

    pub fn save_report_as_json(&self) {
        let result = serde_json::to_string(&self).unwrap();
        let mut file = File::create(self.image.report_path()).unwrap();
        file.write_all(result.as_ref()).unwrap();
    }

    pub fn create_report_from_json(image: &Image) -> serde_json::Result<AnalysisReport> {
        let file = File::open(image.report_path()).unwrap();
        let mut buf_reader = BufReader::new(file);

        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        serde_json::from_str(&contents)
    }

    pub fn show_report(&self) {
        println!();
        println!("{}", bold("Analysis Report:"));
        println!("  Efficiency score: {} %", self.color_score());
        println!("  Total size: {}", size_human(self.total_space),);
        println!("  Wasted Space: {}", size_human(self.wasted_space),);

        println!();
        println!("{}", bold("Inefficient Files:"));
        println!("Count  Wasted Space  File Path");
        for i in self.dup_files.iter() {
            println!(
                "{:>5}  {:>12}  {}",
                i.count,
                size_human(i.wasted_size),
                i.path
            );
        }

        println!();
        println!("{}", bold("Packages:"));
        for mngr in self.managers.iter() {
            mngr.show_report();
        }
    }

    pub fn show_packages(&self) {
        println!();
        println!("{}", bold("Packages:"));
        for mngr in self.managers.iter() {
            mngr.show_packages();
        }
    }

    fn color_score(&self) -> String {
        let score_str = self.score.to_string();
        if self.score > 90 {
            return green(&score_str);
        } else if self.score > 70 {
            return yellow(&score_str);
        }
        red(&score_str)
    }
}
