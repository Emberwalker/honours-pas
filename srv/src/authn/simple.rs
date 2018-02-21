use std::sync::Arc;
use diesel;
use diesel::prelude::*;
use schema::authn_credentials;
use db::Pool;
use super::{AuthnBackend, AuthnCreateError, AuthnFailure};
use util;

// Diesel structs for authn_credentials
#[derive(Debug, Queryable)]
pub struct AuthnCredential {
    email: String,
    login_email: String,
    password: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "authn_credentials"]
pub struct NewAuthnCredential<'a> {
    email: &'a str,
    login_email: &'a str,
    password: &'a str,
}

pub struct SimpleAuthnBackend {
    pool: Arc<Pool>,
}

impl<'a> AuthnBackend for SimpleAuthnBackend {
    fn new(_config_location: &str, pool: Arc<Pool>) -> Self {
        SimpleAuthnBackend {
            pool: pool,
        }
    }

    fn authenticate(&self, username: &str, passwd: &str) -> Result<String, AuthnFailure> {
        use schema::authn_credentials::dsl::*;

        let login = sanitise_email(username).map_err(|_| AuthnFailure::InvalidUser())?;

        let conn = self.pool.get().map_err(|e| {
            error!("Error fetching connection from pool: {}", e);
            AuthnFailure::Error()
        })?;

        let usr = authn_credentials
            .filter(login_email.eq(login))
            .load::<AuthnCredential>(&*conn)
            .map_err(|e| {
                error!("Error accessing database: {}", e);
                AuthnFailure::Error()
            })?;

        if let Some(entry) = usr.get(0) {
            if entry.password.is_none() {
                return Err(AuthnFailure::InvalidUser());
            }

            let srv_passwd = entry.password.as_ref().unwrap();
            if util::check_password(passwd, &srv_passwd) {
                return Ok(entry.email.clone());
            } else {
                return Err(AuthnFailure::InvalidPassword());
            }
        }

        Err(AuthnFailure::InvalidUser())
    }

    fn create_user(&self, username: &str, passwd: &str) -> Result<(), AuthnCreateError> {
        use schema::authn_credentials::dsl::*;

        let pwd_gen = util::hash_password(passwd).map_err(|e| {
            error!("Error generating password hash: {}", e);
            AuthnCreateError::Other()
        })?;

        /*let username = username.to_lowercase();
        let mut matches = username.splitn(2, "@");
        let err_closure = || {
            warn!("Error parsing username as email: '{}'", username);
            AuthnCreateError::Other()
        };
        let u1 = matches.next().ok_or_else(&err_closure)?;
        let u2 = matches.next().ok_or_else(&err_closure)?;
        let login = format!("{}@{}", u1.replace(".", ""), u2);*/

        let login = sanitise_email(username).map_err(|_| AuthnCreateError::Other())?;
        let username = username.to_lowercase();

        let new_user = NewAuthnCredential {
            email: username.as_str(),
            login_email: login.as_str(),
            password: pwd_gen.as_str(),
        };

        let conn = self.pool.get().map_err(|e| {
            error!("Error fetching connection from pool: {}", e);
            AuthnCreateError::DatabaseFailure()
        })?;

        diesel::insert_into(authn_credentials).values(&new_user).execute(&*conn).map_err(|e| {
            error!("Database error when inserting new user: {}", e);
            AuthnCreateError::DatabaseFailure()
        })?;
        
        Ok(())
    }
}

fn sanitise_email(uname: &str) -> Result<String, ()> {
    let username = uname.to_lowercase();
    let mut matches = username.splitn(2, "@");
    let err_closure = || {
        warn!("Error parsing username as email: '{}'", username);
        ()
    };
    let u1 = matches.next().ok_or_else(&err_closure)?;
    let u2 = matches.next().ok_or_else(&err_closure)?;
    Ok(format!("{}@{}", u1.replace(".", ""), u2))
}
