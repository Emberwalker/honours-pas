use std::sync::Arc;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use ldap3::{ldap_escape, LdapConn, Scope, SearchEntry};
use toml;
use db::Pool;
use super::{AuthnBackend, AuthnCreateError, AuthnFailure};
use util;

lazy_static! {
    static ref VALID_USERNAME_REGEX: Regex = Regex::new(r"^[\w\d\.@]+$").unwrap();
}

#[derive(Deserialize, Debug)]
struct ConfigRoot {
    ldap: LdapConfig,
}

#[derive(Deserialize, Debug)]
struct LdapConfig {
    server_url: String,
    search_base: String,
    filter_field: Option<String>,
    email_field: Option<String>,
    domain: Option<String>,
    is_ad: Option<bool>,
    normalize_logins: Option<bool>,
}

impl LdapConfig {
    fn to_backend(self) -> LdapAuthnBackend {
        let ad = self.is_ad.unwrap_or(false);
        if ad && self.domain.is_none() {
            error!("LDAP configuration specifies an AD server, but no AD domain.");
            panic!("LDAP configuration specifies an AD server, but no AD domain.");
        }

        LdapAuthnBackend {
            server_url: self.server_url,
            search_base: self.search_base,
            filter_field: self.filter_field.unwrap_or_else(|| {
                if ad {
                    "sAMAccountName".to_string()
                } else {
                    "userPrincipalName".to_string()
                }
            }),
            email_field: self.email_field.unwrap_or_else(|| "mail".to_string()),
            domain: self.domain,
            is_ad: ad,
            normalize_logins: self.normalize_logins.unwrap_or(false),
        }
    }
}

pub struct LdapAuthnBackend {
    server_url: String,
    search_base: String,
    filter_field: String,
    email_field: String,
    domain: Option<String>,
    is_ad: bool,
    normalize_logins: bool,
}

enum LdapAuthnError {
    ConnectionError(),
    InvalidLogin(),
    Other(),
}

impl LdapAuthnBackend {
    pub fn new(config_location: &str, _pool: Arc<Pool>) -> Self {
        let res: Result<ConfigRoot, toml::de::Error> = {
            info!("Loading LDAP configuration from {}", config_location);
            let mut f = File::open(config_location).unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            toml::from_str(&contents[..])
        };

        if res.is_err() {
            error!("Unable to parse LDAP config; is a field missing?");
            panic!(res.unwrap_err());
        }

        res.unwrap().ldap.to_backend()
    }

    fn attempt_authn(&self, uname: &str, passwd: &str) -> Result<String, LdapAuthnError> {
        if !VALID_USERNAME_REGEX.is_match(uname) {
            return Err(LdapAuthnError::InvalidLogin());
        }

        let mut username = match self.normalize_logins {
            true => util::sanitise_email(&uname.to_lowercase()).map_err(|_e| LdapAuthnError::InvalidLogin())?,
            false => uname.to_string(),
        };
        let mut uid = username.clone();

        let original_uname = username.clone();

        if self.is_ad {
            if let Some(ref domain) = self.domain {
                if let Some(pos) = username.find("@") {
                    username = username[0..pos].to_string();
                    uid = username.clone();
                }
                username = format!("{}\\{}", domain, username);
            }
        }

        let conn = LdapConn::new(&self.server_url).map_err(|_e| {
            error!("Unable to connect to directory: {}", self.server_url);
            LdapAuthnError::ConnectionError()
        })?;

        conn
            .simple_bind(&username, passwd)
            .map_err(|_e| LdapAuthnError::InvalidLogin())?
            .success()
            .map_err(|_e| LdapAuthnError::InvalidLogin())?;

        let filter = format!("({}={})", self.filter_field, ldap_escape(uid));
        let (results, _meta) = conn
            .search(&self.search_base, Scope::Subtree, &filter, vec!(self.email_field.clone()))
            .map_err(|e| {
                warn!("Error fetching user row: {}", e);
                LdapAuthnError::Other()
            })?
            .success()
            .map_err(|e| {
                warn!("Error fetching user row: {}", e);
                LdapAuthnError::Other()
            })?;

        if results.len() == 0 {
            error!("LDAP returned no user! {}", original_uname);
            return Err(LdapAuthnError::Other());
        }

        if results.len() > 1 {
            warn!("LDAP returned multiple entries; assuming first.");
        }

        let raw_res = results.get(0).unwrap();
        let res = SearchEntry::construct(raw_res.clone());

        res.attrs[&self.email_field].get(0).ok_or_else(|| {
            error!("LDAP record didn't contain a mail field.");
            LdapAuthnError::Other()
        }).map(|it| it.clone())
    }
}

impl AuthnBackend for LdapAuthnBackend {
    fn authenticate(&self, username: &str, passwd: &str) -> Result<String, AuthnFailure> {
        match self.attempt_authn(username, passwd) {
            Ok(s) => Ok(s),
            Err(LdapAuthnError::ConnectionError()) => Err(AuthnFailure::Error()),
            Err(LdapAuthnError::InvalidLogin()) => Err(AuthnFailure::InvalidUserOrPassword()),
            Err(LdapAuthnError::Other()) => Err(AuthnFailure::Error()),
        }
    }

    fn create_user(&self, _user: &str, _pass: &str) -> Result<(), AuthnCreateError> {
        Err(AuthnCreateError::ActionNotSupported())
    }
}
