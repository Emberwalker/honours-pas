#![feature(custom_derive)] // TODO: Remove this when Rocket switches fully to `proc_macro`
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(unknown_lints)]
#![warn(clippy)]
#![allow(print_literal)]

#[macro_use]
extern crate downcast_rs;
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
extern crate num_traits;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate time;

extern crate jsonwebtoken;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate base64;
extern crate openssl;
extern crate toml;
extern crate url;

extern crate ldap3;
extern crate reqwest;
extern crate ring_pwhash;
extern crate rocket;
extern crate rocket_contrib;

use authn::AuthnBackend;
use rocket::config::{Config, Environment, LoggingLevel};
use std::sync::Arc;
use std::{thread, time as std_time};

#[macro_use]
mod util;
mod authn;
mod config;
mod controller;
mod db;
mod fairing;
mod migrate;
mod schema;
mod session;

#[cfg(feature = "insecure")]
fn get_rocket_config(conf: &config::Config) -> Config {
    let mut b = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(8080)
        .log_level(LoggingLevel::Debug);
    if let Some(key) = conf.get_secret_key() {
        b = b.secret_key(key);
    }
    b.finalize().expect("Config builder")
}

#[cfg(not(feature = "insecure"))]
fn get_rocket_config(conf: &config::Config) -> Config {
    let mut b = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(8080)
        .log_level(LoggingLevel::Critical);
    if let Some(key) = conf.get_secret_key() {
        b = b.secret_key(key);
    }
    b.finalize().expect("Config builder")
}

fn get_authn_provider(
    conf_loc: &str,
    conf: &config::Config,
    pool: Arc<db::Pool>,
) -> Arc<AuthnBackend> {
    match conf.get_authn_provider().as_str() {
        "simple" => Arc::new(authn::simple::SimpleAuthnBackend::new(conf_loc, pool)),
        "ldap" => Arc::new(authn::ldap::LdapAuthnBackend::new(conf_loc, pool)),
        "aad" | "openid" => authn::openid::OpenIDAuthnBackend::new(conf_loc, conf),
        s => {
            error!("No such authn backend: {}", s);
            panic!("No such authn backend: {}", s);
        }
    }
}

fn get_conf(conf_loc: &str) -> config::Config {
    // TODO: More nuanced config error handling (like logging what keys had to be defaulted)
    config::load_config(conf_loc).unwrap_or_else(|_| config::default_config())
}

fn run_migrations(conf: &config::Config) {
    debug!("Using configuration: {:?}", conf);
    info!("Running database migrations (if needed)...");
    let mut okay = false;
    let sleep_time = std_time::Duration::new(10, 0);
    while !okay {
        if let Err(e) = migrate::run_pending_migrations(conf) {
            warn!("Unable to connect to database server, retrying in 10s: {}", e);
            thread::sleep(sleep_time);
        } else {
            okay = true;
        }
    }
    info!("Database migrations check completed.");
}

pub fn run(conf_loc: &str) -> Result<(), String> {
    let conf = get_conf(conf_loc);
    run_migrations(&conf);

    let pool = Arc::new(db::init_pool(&conf));
    let auth_provider = get_authn_provider(conf_loc, &conf, Arc::clone(&pool));
    let session_provider = session::SessionManager::new(&conf, Arc::clone(&auth_provider));

    rocket::custom(get_rocket_config(&conf), true)
        .attach(fairing::ServerHeader())
        .catch(controller::v1::get_catchers(&conf))
        .mount("/api/v1", controller::v1::get_routes(&conf))
        .mount("/api/authn", auth_provider.get_rocket_routes())
        .manage(authn::AuthnHolder(Arc::clone(&auth_provider)))
        .manage(pool)
        .manage(session_provider)
        .manage(conf)
        .launch();
    Ok(())
}

pub fn add_user(conf_loc: &str, uname: &str, passwd: &str, fname: &str) -> Result<(), String> {
    use db::models::new::Staff as NewStaff;
    use db::staff;

    let conf = get_conf(conf_loc);
    run_migrations(&conf);

    let pool = Arc::new(db::init_pool(&conf));
    let auth_provider = Arc::new(authn::simple::SimpleAuthnBackend::new(conf_loc, Arc::clone(&pool)));

    auth_provider
        .create_user(uname, passwd)
        .map_err(|e| format!("{:?}", e))?;
    staff::create(
        &db::DatabaseConnection(pool.get().unwrap()),
        &NewStaff {
            email: uname.to_string(),
            full_name: fname.to_string(),
            is_admin: Some(true),
        },
    ).map_err(|e| format!("{:?}", e))?;

    Ok(())
}
