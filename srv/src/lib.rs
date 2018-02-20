#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod config;
mod schema;
mod migrate;

pub fn run(conf_loc: &str) -> Result<(), String> {
    let conf = config::load_config(conf_loc).unwrap_or(config::default_config());
    debug!("CFG: {:?}", conf);
    if let Err(e) = migrate::run_pending_migrations(&conf) {
        panic!(e);
    }
    Ok(())
}
