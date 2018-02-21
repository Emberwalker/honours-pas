use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2;

use super::config::Config;

// The following is based on Rocket's guide on integrating DB connection pooling.
// https://rocket.rs/guide/state/#databases

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(conf: &Config) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(format!("postgres://{}", conf.database_string));
    r2d2::Pool::new(manager).expect("DB pool creation")
}
