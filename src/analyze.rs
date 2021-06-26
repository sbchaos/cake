use log::{info, trace};

use crate::analysis::pkg_manager::Manager;
use crate::analysis::report::AnalysisReport;
use crate::docker;
use crate::image::image_tar::{extract_image_tar, read_tar_layer};
use crate::image::manifest::Manifest;
use crate::image::Image;
use crate::ofs::layer::Layer;
use crate::ofs::ofs::OverlayFs;
use crate::packages::alpine::AlpinePackageManager;
use crate::packages::apt::DebianPackageManager;
use crate::packages::archive::ArchiveManager;
use crate::packages::deps::PackageManager;
use crate::packages::rpm::RPMPackageManager;
use std::path::Path;

pub fn analyze_image(image: Image, pkgs: bool, tree: bool) {
    let report = match read_report(&image) {
        Some(report) => report,
        None => {
            let overlayfs = create_ofs(&image);
            create_analysis_report(overlayfs, image)
        }
    };

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

fn read_report(image: &Image) -> Option<AnalysisReport> {
    let image_json = image.report_path();
    if Path::new(&image_json).exists() {
        trace!("Loaded analysis report from cache: {}", image_json);
        return Some(AnalysisReport::create_report_from_json(image));
    }
    None
}

fn create_ofs(image: &Image) -> OverlayFs {
    let mut overlayfs = OverlayFs::new();

    if Path::new(&image.tree_path()).exists() {
        overlayfs = OverlayFs::create_fs_from_json(image);
        trace!("Loaded from cache: {}", image.tree_path());
    } else {
        if !Path::new(&image.dir_path()).exists() {
            fetch(image);
            extract_image_tar(image);
        }

        let manifest = Manifest::for_image_path(image).unwrap();
        for (i, layer_path) in manifest.layers.iter().enumerate() {
            let path = format!("{}/{}", image.image_id, layer_path);
            trace!("path : {}", path);
            let layer_id = format!("layer{}", i);
            read_tar_layer(&mut overlayfs, &path, &layer_id);

            overlayfs.add_layer(Layer::new(layer_id, path));
        }

        overlayfs.update_sizes();
        overlayfs.save_tree_to_json(image);
    }
    overlayfs
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
