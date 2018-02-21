#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate rocket;
extern crate rocket_contrib;

mod config;
mod schema;
mod migrate;
mod db;

pub fn run(conf_loc: &str) -> Result<(), String> {
    let conf = config::load_config(conf_loc).unwrap_or(config::default_config());
    debug!("CFG: {:?}", conf);
    info!("Running database migrations (if needed)...");
    if let Err(e) = migrate::run_pending_migrations(&conf) {
        panic!("Error running DB migrations: {}", e);
    }
    info!("Database migrations check completed.");

    rocket::ignite().manage(db::init_pool(&conf)).launch();
    Ok(())
}
