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
        .get_matches();

    logs::setup_logging(&matches);

    let image = matches.value_of("IMAGE").unwrap();
    trace!("Using IMAGE file: {}", image);

    analyze::analyze_image(image);
}
