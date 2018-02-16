#[macro_use]
extern crate clap;
extern crate pas_backend;

use clap::{Arg, App, SubCommand};

static CONF_LOC_ENV: &'static str = "HONOURS_PAS_CONF"
static DEFAULT_CONF_LOC: &'static str = "/var/run/pas_backend.json"

fn main() {
	let matches = App::new("Honours Project Allocation service backend")
        .version(crate_version!());
        .author("Robert T. <arkan@drakon.io>")
        .about("Backend daemon for the Honours Project Allocation service.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Provide a custom configuration file.")
            .env(CONF_LOC_ENV)
            .default_value(DEFAULT_CONF_LOC)
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Sets verbosity. May be specified up to 4 times."));
}
