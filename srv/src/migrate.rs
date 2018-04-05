use super::config::Config;
use diesel::pg::PgConnection;
use diesel::prelude::*;

embed_migrations!();

pub fn run_pending_migrations(conf: &Config) -> Result<(), ConnectionError> {
    let conn = PgConnection::establish(&conf.get_database_str())?;
    embedded_migrations::run(&conn).expect("apply bundled migrations");
    Ok(())
}
