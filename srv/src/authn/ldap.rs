use std::sync::Arc;
use std::error::Error;
use regex::Regex;
use ldap3::{ldap_escape, LdapConn, Scope, SearchEntry};
use db::Pool;
use super::{AuthnBackend, AuthnCreateError, AuthnFailure};
use util;

lazy_static! {
    static ref VALID_USERNAME_REGEX: Regex = Regex::new(r"^[\w\d\.@]+$").unwrap();
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
        // TODO: Load from config
        LdapAuthnBackend {
            server_url: "ldap://thanatos.mshome.net".to_string(),
            search_base: "dc=test,dc=drakon,dc=io".to_string(),
            filter_field: "sAMAccountName".to_string(),
            email_field: "mail".to_string(),
            domain: Some("TESTNET".to_string()),
            is_ad: true,
            normalize_logins: true,
        }
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
        let (results, meta) = conn
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
