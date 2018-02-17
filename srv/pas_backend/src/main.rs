extern crate chrono;
#[macro_use]
extern crate clap;
extern crate fern;
#[macro_use]
extern crate log;
extern crate pas_backend;

use clap::{App, Arg};

static CONF_LOC_ENV: &'static str = "HONOURS_PAS_CONF";
static DEFAULT_CONF_LOC: &'static str = "/var/run/pas_backend.json";

fn main() {
    let matches = App::new("Honours Project Allocation service backend")
        .version(crate_version!())
        .author("Robert T. <arkan@drakon.io>")
        .about("Backend daemon for the Honours Project Allocation service.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Provide a custom configuration file.")
                .env(CONF_LOC_ENV)
                .default_value(DEFAULT_CONF_LOC)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets verbosity. May be specified up to 4 times."),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("Disables log output. Overrides verbosity flags."),
        )
        .get_matches();

    let conf_loc = matches.value_of("config").unwrap_or(DEFAULT_CONF_LOC);
    let mut log_lvl = match matches.occurrences_of("v") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 | _ => log::LevelFilter::Debug,
    };

    if matches.is_present("quiet") {
        log_lvl = log::LevelFilter::Off;
    }

    if let Err(err) = setup_logger(log_lvl) {
        panic!(format!("Error setting up logger: {}", err));
    }

    info!("Logger configured; using log level {}", log_lvl)
}

fn setup_logger(lvl: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}][{}:{}] {}",
                chrono::Utc::now().format("%Y/%m/%d %H:%M:%S%.3f%z"),
                record.level(),
                record.target(),
                record.file().unwrap_or("<unknown>"),
                record
                    .line()
                    .map(|it| it.to_string())
                    .unwrap_or("???".to_string()),
                message
            ))
        })
        .level(lvl)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
