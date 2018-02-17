extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
#[macro_use]
extern crate log;

mod config;

pub fn run(conf_loc: &str) -> Result<(), String> {
    let conf = config::load_config(conf_loc).unwrap_or(config::default_config());
    debug!("CFG: {:?}", conf);
    Ok(())
}