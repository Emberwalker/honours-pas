extern crate chrono;
#[macro_use]
extern crate clap;
extern crate fern;
extern crate hpas;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;

use clap::{App, Arg, SubCommand};
use regex::Regex;
use std::cmp;

static CONF_LOC_ENV: &'static str = "HONOURS_PAS_CONF";
static DEFAULT_CONF_LOC: &'static str = "/opt/hpas/conf/config.toml";

// From https://docs.rs/console/0.6.1/src/console/utils.rs.html#12
lazy_static! {
    static ref STRIP_ANSI_RE: Regex = Regex::new(
        r"[\x1b\x9b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]"
    ).unwrap();
}

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
                .global(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets verbosity. May be specified up to 4 times.")
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("add_user")
                .about("For 'simple' authn backend: Adds an admin user to the system.")
                .arg(
                    Arg::with_name("username")
                        .short("u")
                        .long("username")
                        .value_name("USERNAME")
                        .help("New user's name. Must be a valid email address.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .value_name("PASSWORD")
                        .help("New user's password.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("fullname")
                        .short("n")
                        .long("name")
                        .value_name("NAME")
                        .help("New user's full name.")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    let conf_loc = matches.value_of("config").unwrap_or(DEFAULT_CONF_LOC);
    let log_lvl = match matches.occurrences_of("v") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        4 | _ => log::LevelFilter::Trace,
    };

    if let Err(err) = setup_logger(log_lvl) {
        panic!(format!("Error setting up logger: {}", err));
    }

    info!(target: "main", "Logger configured; using log level {}", log_lvl);

    if let Some(submatches) = matches.subcommand_matches("add_user") {
        if let Err(e) = hpas::add_user(
            conf_loc,
            submatches.value_of("username").unwrap(),
            submatches.value_of("password").unwrap(),
            submatches.value_of("fullname").unwrap(),
        ) {
            error!("Failed with error: {}", e);
        }
    } else if let Err(e) = hpas::run(conf_loc) {
        error!("Failed with error: {}", e);
    }
}

fn setup_logger(lvl: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            if record.target().starts_with("hpas") || record.target().starts_with("macro_gen") {
                out.finish(format_args!(
                    "[{}][{}][{}][{}:{}] {}",
                    chrono::Utc::now().format("%Y/%m/%d %H:%M:%S%.3f%z"),
                    record.level(),
                    record.target(),
                    record.file().unwrap_or("<unknown>"),
                    record
                        .line()
                        .map(|it| it.to_string())
                        .unwrap_or_else(|| "???".to_string()),
                    message
                ))
            } else {
                // We drop the file info for dependencies, since their file paths are long and absolute.
                // Also strip any ANSI sequences, since Rocket likes to use them (which mucks up stuff after it).
                out.finish(format_args!(
                    "[{}][{}][{}] {}",
                    chrono::Utc::now().format("%Y/%m/%d %H:%M:%S%.3f%z"),
                    record.level(),
                    record.target(),
                    STRIP_ANSI_RE.replace_all(&format!("{}", message), "")
                ))
            }
        })
        .level(lvl)
        // Set Hyper + tokio to WARN or the assigned level, whichever is least verbose.
        .level_for("hyper", cmp::min(log::LevelFilter::Warn, lvl))
        .level_for("tokio_core", cmp::min(log::LevelFilter::Warn, lvl))
        .level_for("tokio_reactor", cmp::min(log::LevelFilter::Warn, lvl))
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
