pub use super::models::Session;
pub use super::models::new::Session as NewSession;

use db::{DatabaseConnection, SelectError};

//generate_create_fn!(sessions, NewSession, Session);

pub fn get_latest_session(conn: &DatabaseConnection) -> Result<Session, SelectError> {
    use diesel;
    use diesel::prelude::*;
    use schema::sessions::dsl::*;

    let res = sessions
        .filter(force_archive.eq(false))
        .order(created.desc())
        .first::<Session>(conn.raw())
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => SelectError::NoSuchValue(),
                e => SelectError::DieselError(e),
            }
        })?;

    Ok(res)
}