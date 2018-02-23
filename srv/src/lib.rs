#![feature(plugin)]
#![feature(conservative_impl_trait)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rand;
extern crate regex;

extern crate bigdecimal;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate ring_pwhash;
extern crate rocket;
extern crate rocket_contrib;

use std::sync::Arc;
use rocket::config::{Config, Environment};
use authn::AuthnBackend;

mod config;
mod schema;
mod migrate;
mod db;
mod controller;
mod authn;
mod util;
mod session;

#[cfg(feature = "insecure")]
fn get_rocket_config(conf: &config::Config) -> Config {
    let mut b = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(8080);
    if let Some(key) = conf.get_secret_key() {
        b = b.secret_key(key);
    }
    b.finalize().expect("Config builder")
}

#[cfg(not(feature = "insecure"))]
fn get_rocket_config(conf: &config::Config) -> Config {
    let mut b = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(8080);
    if let Some(key) = conf.get_secret_key() {
        b = b.secret_key(key);
    }
    b.finalize().expect("Config builder")
}

fn get_authn_provider(conf_loc: &str, pool: Arc<db::Pool>) -> Box<AuthnBackend> {
    Box::new(authn::simple::SimpleAuthnBackend::new(conf_loc, pool))
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

    let pool = Arc::new(db::init_pool(&conf));
    let auth_provider = get_authn_provider(conf_loc, Arc::clone(&pool));
    let session_provider = session::SessionManager::new(&conf);

    rocket::custom(get_rocket_config(&conf), true)
        .manage(authn::AuthnHolder(auth_provider))
        .manage(pool)
        .manage(session_provider)
        .mount("/api/v1", controller::v1::get_routes(&conf))
        .launch();
    Ok(())
}
