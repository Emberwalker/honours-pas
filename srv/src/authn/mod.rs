use db::Pool;

pub trait AuthnBackend<'a> {
    /// Called before the backend is actually used. This is useful for loading configs from the global config file, and
    /// for saving a reference to the database pool (if required). Making use of Serde and toml-rs is recommended.
    fn init(config_location: &str, pool: &'a Pool);

    /// Called when the system attempts to authenticate a user. Returns the users externally-visible email address if
    /// authentication succeeds.
    fn authenticate(username: &str, password: &str) -> Option<String>;
}
