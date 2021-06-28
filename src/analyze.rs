use log::{info, trace};

use crate::analysis::pkg_manager::Manager;
use crate::analysis::report::AnalysisReport;
use crate::docker;
use crate::image::image_tar::{extract_image_tar, read_tar_layer};
use crate::image::manifest::Manifest;
use crate::image::{Image, Source};
use crate::ofs::layer::Layer;
use crate::ofs::ofs::OverlayFs;
use crate::packages::alpine::AlpinePackageManager;
use crate::packages::apt::DebianPackageManager;
use crate::packages::archive::ArchiveManager;
use crate::packages::deps::PackageManager;
use crate::packages::rpm::RPMPackageManager;

pub fn analyze_image(image: Image, pkgs: bool, tree: bool) {
    let report = generate_analysis_report(image);

    if tree {
        report.ofs.show_as_tree();
        return;
    }

    if pkgs {
        report.show_packages();
    } else {
        report.show_report();
    }
}

fn generate_analysis_report(image: Image) -> AnalysisReport {
    match image.source {
        Source::Report => {
            if let Ok(report) = AnalysisReport::create_report_from_json(&image) {
                return report;
            }
        }
        Source::Tree => {
            if let Ok(ofs) = OverlayFs::create_fs_from_json(&image) {
                return create_analysis_report(ofs, image);
            }
        }
        Source::Dir => {}
        Source::Tar => {
            extract_image_tar(&image);
        }
        Source::Docker => {
            fetch(&image);
            extract_image_tar(&image);
        }
        Source::None => {
            unreachable!("Should not generate report for no image")
        }
    };

    let mut overlayfs = OverlayFs::new();

    let manifest = Manifest::for_image_path(&image.image_id).unwrap();
    trace!("manifest: {:?}", manifest);
    for (i, layer_path) in manifest.layers.iter().enumerate() {
        let path = format!("{}/{}", image.image_id, layer_path);
        trace!("path : {}", path);
        let layer_id = format!("layer{}", i);
        read_tar_layer(&mut overlayfs, &path, &layer_id);

        overlayfs.add_layer(Layer::new(layer_id, path));
    }

    overlayfs.update_sizes();
    overlayfs.save_tree_to_json(&image);

    create_analysis_report(overlayfs, image)
}

fn fetch(image: &Image) {
    info!("Fetching image docker://{}", image.name);
    info!("Can take a while for larger images");

    docker::save(&image.image_id);
    trace!("Image saved");
}

fn create_analysis_report(ofs: OverlayFs, image: Image) -> AnalysisReport {
    let alpine_pkg = AlpinePackageManager::new(&ofs);
    let rpm_pkg = RPMPackageManager::new(&ofs, &image.image_id);
    let deb_pkg = DebianPackageManager::new(&ofs);
    let archive_mgr = ArchiveManager::new(&ofs);

    let pkg_managers = vec![
        Manager {
            name: alpine_pkg.name().to_string(),
            cache: alpine_pkg.get_cache(),
            waste_size: alpine_pkg.get_wasted_bytes(),
            packages: alpine_pkg.get_installed_packages(),
        },
        Manager {
            name: rpm_pkg.name().to_string(),
            cache: rpm_pkg.get_cache(),
            waste_size: rpm_pkg.get_wasted_bytes(),
            packages: rpm_pkg.get_installed_packages(),
        },
        Manager {
            name: deb_pkg.name().to_string(),
            cache: deb_pkg.get_cache(),
            waste_size: deb_pkg.get_wasted_bytes(),
            packages: deb_pkg.get_installed_packages(),
        },
        Manager {
            name: archive_mgr.name().to_string(),
            cache: archive_mgr.get_cache(),
            waste_size: archive_mgr.get_wasted_bytes(),
            packages: archive_mgr.get_installed_packages(),
        },
    ];

    let report = AnalysisReport::create_analysis_report(ofs, image, pkg_managers);
    report.save_report_as_json();
    report
}
