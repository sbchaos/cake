use clap::{App, Arg};
use log::trace;

mod analysis;
mod analyze;
mod docker;
mod image;
mod logs;
mod ofs;
mod packages;
mod style;

fn main() {
    let matches = App::new("cake")
        .version(clap::crate_version!())
        .arg(
            Arg::new("loglevel")
                .short('l')
                .long("loglevel")
                .value_name("LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace"])
                .takes_value(true),
        )
        .arg(
            Arg::new("IMAGE")
                .about("the input image to use")
                .required(true),
        )
        .arg(
            Arg::new("packages")
                .about("the list of packages installed")
                .short('p')
                .long("packages")
                .takes_value(false),
        ).arg(
            Arg::new("tree")
                .short('t')
                .long("tree")
                .hidden(true)
                .takes_value(false),
        )
        .get_matches();

    logs::setup_logging(&matches);

    let image = matches.value_of("IMAGE").unwrap();
    trace!("Using IMAGE file: {}", image);

    let pkgs = matches.is_present("packages");
    let tree = matches.is_present("tree");

    analyze::analyze_image(image, pkgs, tree);
}
