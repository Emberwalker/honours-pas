use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    hpas: ConfigHPAS,
    session: Option<SessionConfig>,
}

#[derive(Deserialize, Debug)]
struct ConfigHPAS {
    database_string: String,
    secret_key: Option<String>,
    authn_provider: Option<String>,
    server_address: String,
}

#[derive(Deserialize, Debug)]
struct SessionConfig {
    pub expiry_minutes: u32,
}

impl Config {
    pub fn get_database_str(&self) -> String {
        format!("postgres://{}", self.hpas.database_string)
    }

    pub fn get_secret_key(&self) -> Option<String> {
        match self.hpas.secret_key {
            None => None,
            Some(ref key) => Some(key.clone()),
        }
    }

    pub fn get_session_expiry(&self) -> u32 {
        match self.session {
            Some(ref session) => session.expiry_minutes,
            None => SessionConfig::default().expiry_minutes,
        }
    }

    pub fn get_authn_provider(&self) -> String {
        match self.hpas.authn_provider {
            None => "simple".to_string(),
            Some(ref prov) => prov.to_lowercase(),
        }
    }

    pub fn get_server_address(&self) -> String {
        self.hpas.server_address.trim_right_matches('/').to_string()
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            expiry_minutes: 120,
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    IO(io::Error),
    TOML(toml::de::Error),
}

impl<'a> Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::IO(ref err) => err.description(),
            ConfigError::TOML(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::IO(ref err) => Some(err),
            ConfigError::TOML(ref err) => Some(err),
        }
    }
}

impl<'a> From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IO(err)
    }
}

impl<'a> From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::TOML(err)
    }
}

impl<'a> fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config Error: {}", self.description())
    }
}

pub fn load_config(location: &str) -> Result<Config, ConfigError> {
    info!("Loading configuration from {}", location);
    let mut f = match File::open(location) {
        Err(e) => panic!("Unable to load config file {}: {}", location, e),
        Ok(f) => f,
    };
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(toml::from_str(&contents[..])?)
}

pub fn default_config() -> Config {
    Config {
        hpas: ConfigHPAS {
            database_string: "postgres:banana@postgres/postgres".to_string(),
            secret_key: None,
            authn_provider: None,
            server_address: "http://localhost:8888".to_string(),
        },
        session: None,
    }
}
