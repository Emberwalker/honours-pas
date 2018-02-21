#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate rocket;
extern crate rocket_contrib;

use rocket::config::{Config, Environment};

mod config;
mod schema;
mod migrate;
mod db;
mod controller;

#[cfg(debug_assertions)]
fn get_rocket_config(conf: &config::Config) -> Config {
    let mut b = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(8080)
        .root("/api");
    if let Some(key) = conf.get_secret_key() {
        b = b.secret_key(key);
    }
    b.finalize().expect("Config builder")
}

#[cfg(not(debug_assertions))]
fn get_rocket_config(conf: &config::Config) -> Config {
    let mut b = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(8080)
        .root("/api");
    if let Some(key) = conf.get_secret_key() {
        b = b.secret_key(key);
    }
    b.finalize().expect("Config builder")
}

pub fn run(conf_loc: &str) -> Result<(), String> {
    // TODO: More nuanced config error handling (like logging what keys had to be defaulted)
    let conf = config::load_config(conf_loc).unwrap_or(config::default_config());
    debug!("Using configuration: {:?}", conf);
    info!("Running database migrations (if needed)...");
    if let Err(e) = migrate::run_pending_migrations(&conf) {
        panic!("Error running DB migrations: {}", e);
    }
    info!("Database migrations check completed.");

    rocket::custom(get_rocket_config(&conf), true)
        .manage(db::init_pool(&conf))
        .mount("/v1", controller::v1::get_routes(&conf))
        .launch();
    Ok(())
}
