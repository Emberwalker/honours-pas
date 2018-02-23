use rocket::Route;

use std::ops::Deref;
use std::sync::Arc;
use db::Pool;

// `simple` is the default database-backed provider. It does nothing fancy.
pub mod simple;

#[derive(Debug)]
pub enum AuthnFailure {
    /// An invalid user was passed.
    InvalidUser(),
    /// An invalid password was passed.
    InvalidPassword(),
    /// Some error occured while performing the check e.g. database error.
    Error()
}

// TODO: Add more error types as they become apparent.
#[derive(Debug)]
#[allow(dead_code)]
pub enum AuthnCreateError {
    ActionNotSupported(),
    DatabaseFailure(),
    NetworkFailure(),
    Other(),
}

pub trait AuthnBackend: Send + Sync {
    /// Provides a set of Rocket routes. These will be mounted at "/api/authn", for tasks such as e.g. email
    /// verification endpoints. On success, ideally redirect back to "/".If no routes are required, return an empty vec.
    /// When generating the vector, it's probably best to use the Rocket `routes![]` macro.
    fn get_rocket_routes(&self) -> Vec<Route> {
        Vec::new()
    }

    /// Called when the system attempts to authenticate a user. Returns the users externally-visible email address if
    /// authentication succeeds, else AuthnFailure::InvalidUser or AuthnFailure::InvalidPassword.
    fn authenticate(&self, username: &str, password: &str) -> Result<String, AuthnFailure>;

    /// Creates a new user (if the backend supports it).
    fn create_user(&self, _username: &str, _password: &str) -> Result<(), AuthnCreateError> {
        Err(AuthnCreateError::ActionNotSupported())
    }
}

pub struct AuthnHolder<'a> (pub Box<AuthnBackend + 'a>);

impl<'a> AuthnHolder<'a> {
    pub fn get(&'a self) -> &'a AuthnBackend {
        &*self.0
    }
}

impl<'a> AuthnBackend for AuthnHolder<'a> {
    fn get_rocket_routes(&self) -> Vec<Route> {
        self.0.get_rocket_routes()
    }

    fn authenticate(&self, username: &str, password: &str) -> Result<String, AuthnFailure> {
        self.0.authenticate(username, password)
    }

    fn create_user(&self, username: &str, password: &str) -> Result<(), AuthnCreateError> {
        self.0.create_user(username, password)
    }
}