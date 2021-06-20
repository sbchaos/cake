use clap::ArgMatches;
use log::LevelFilter;

pub fn setup_logging(matches: &ArgMatches) {
    let loglevel = match matches.value_of("loglevel") {
        None => LevelFilter::Warn,
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => unreachable!(),
    };

    let mut builder = env_logger::builder();
    builder.filter_level(loglevel);

    builder.try_init().expect("Could not initialize logging")
}
