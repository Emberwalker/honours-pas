use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    database_string: String,
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
    let mut f = File::open(location)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(toml::from_str(&contents[..])?)
}

pub fn default_config() -> Config {
    Config {
        database_string: "postgres://postgres:banana@postgres/hpas".to_string(),
    }
}