use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::config::Config;

embed_migrations!();

pub fn run_pending_migrations(conf: &Config) -> Result<(), ConnectionError> {
    let srv_url = format!("postgres://{}", conf.database_string);
    let conn = PgConnection::establish(&srv_url)?;
    embedded_migrations::run(&conn).expect("Error applying bundled migrations.");
    Ok(())
}