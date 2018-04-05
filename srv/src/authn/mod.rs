use std::sync::Arc;

use downcast_rs::Downcast;
use rocket::Route;
use serde_json::Value;

// `simple` is the default database-backed provider. It does nothing fancy.
pub mod simple;
// `ldap` is the LDAP/AD-based provider.
pub mod ldap;
// `openid` is the OpenID Connect provider (useful for Azure AD)
pub mod openid;

#[derive(Debug)]
pub enum AuthnFailure {
    /// An invalid user was passed.
    InvalidUser(),
    /// An invalid password was passed.
    InvalidPassword(),
    /// An invalid set of credentials. Returned by schemes which cannot tell whether username or password is wrong.
    InvalidUserOrPassword(),
    /// Action not supported by backend (e.g. OAuth 2.0 doesn't use a username/password)
    NotSupported(),
    /// Some error occured while performing the check e.g. database error.
    Error(),
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

pub trait AuthnBackend: Downcast + Send + Sync {
    /// Provides a set of Rocket routes. These will be mounted at "/api/authn", for tasks such as e.g. email
    /// verification endpoints. On success, ideally redirect back to "/". Only implement if needing routes.
    /// When generating the vector, it's probably best to use the Rocket `routes![]` macro.
    fn get_rocket_routes(&self) -> Vec<Route> {
        routes![]
    }

    /// Called when the system attempts to authenticate a user. Returns the users externally-visible email address if
    /// authentication succeeds, else AuthnFailure::InvalidUser or AuthnFailure::InvalidPassword.
    fn authenticate(&self, username: &str, password: &str) -> Result<String, AuthnFailure>;

    /// Creates a new user (if the backend supports it).
    fn create_user(&self, _username: &str, _password: &str) -> Result<(), AuthnCreateError> {
        Err(AuthnCreateError::ActionNotSupported())
    }

    /// Allows adding metadata to the client metadata endpoint.
    fn add_to_client_meta(&self, _meta: &mut Value) {}

    /// Allows custom on-logout functionality in providers.
    fn on_logout(&self, _email: &str) -> Option<String> {
        None
    }
}

// Enable (safe) downcasting to implementing types.
// See https://crates.io/crates/downcast-rs
#[allow(dead_code)]
impl_downcast!(AuthnBackend);

pub struct AuthnHolder(pub Arc<AuthnBackend>);

impl AuthnBackend for AuthnHolder {
    fn get_rocket_routes(&self) -> Vec<Route> {
        self.0.get_rocket_routes()
    }

    fn authenticate(&self, username: &str, password: &str) -> Result<String, AuthnFailure> {
        self.0.authenticate(username, password)
    }

    fn create_user(&self, username: &str, password: &str) -> Result<(), AuthnCreateError> {
        self.0.create_user(username, password)
    }

    fn add_to_client_meta(&self, meta: &mut Value) {
        self.0.add_to_client_meta(meta);
    }

    fn on_logout(&self, email: &str) -> Option<String> {
        self.0.on_logout(email)
    }
}
