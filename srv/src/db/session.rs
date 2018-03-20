pub use super::models::Session;
pub use super::models::new::Session as NewSession;

use db::{DatabaseConnection, SelectError};

generate_crud_fns!(sessions, NewSession, Session);

/// Fetches a session from the database along with whether it's the current session.
pub fn get_session(conn: &DatabaseConnection, id: i32) -> Result<(bool, Session), SelectError> {
    match get_latest_session(conn) {
        Ok(ref s) if s.id == id => return Ok((true, s.clone())),
        Ok(_) => (),
        Err(SelectError::NoSuchValue()) => return Err(SelectError::NoSuchValue()),
        Err(e @ SelectError::DieselError(_)) => return Err(e),
    }

    let res = generate_select_body!(single, conn, sessions, Session, (id, id))?;
    Ok((false, res))
}

pub fn get_latest_session(conn: &DatabaseConnection) -> Result<Session, SelectError> {
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

pub fn get_all(conn: &DatabaseConnection) -> Result<Vec<(bool, Session)>, SelectError> {
    use diesel::prelude::*;
    use schema::sessions::dsl::*;

    let res = sessions
        .order(created.desc())
        .load::<Session>(conn.raw())
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => SelectError::NoSuchValue(),
                e => SelectError::DieselError(e),
            }
        })?;
    
    let mut first = true;
    Ok(res.into_iter().map(|it| {
        let out = if first && !it.force_archive {
            (true, it)
        } else {
            (false, it)
        };
        first = false;
        out
    }).collect())
}