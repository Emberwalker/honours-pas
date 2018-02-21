use rocket::Route;

use db::Pool;

pub trait AuthnBackend<'a> {
    /// Called before the backend is actually used. This is useful for loading configs from the global config file, and
    /// for saving a reference to the database pool (if required). Making use of Serde and toml-rs is recommended.
    fn init(config_location: &str, pool: &'a Pool);

    /// Provides a set of Rocket routes. These will be mounted at "/api/authn", for tasks such as e.g. email
    /// verification endpoints. On success, ideally redirect back to "/".If no routes are required, return an empty vec.
    /// When generating the vector, it's probably best to use the Rocket `routes![]` macro.
    fn get_rocket_routes() -> Vec<Route> {
        Vec::new()
    }

    /// Called when the system attempts to authenticate a user. Returns the users externally-visible email address if
    /// authentication succeeds, else None.
    fn authenticate(username: &str, password: &str) -> Option<String>;
}
