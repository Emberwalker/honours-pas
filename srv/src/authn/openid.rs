use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use rocket::Route;
use serde_json::Value;
use toml;
use reqwest;
use jsonwebtoken as jwt;

use config::Config as HPASConfig;
use super::{AuthnBackend, AuthnFailure};

lazy_static! {
    static ref CSRF_DURATION: Duration = Duration::from_secs(5*60); // 5 minutes
    static ref HTTP: reqwest::Client = reqwest::Client::new();
}

#[derive(Deserialize, Debug)]
struct Config {
    pub aad: Option<AADInnerConfig>,
    pub openid: Option<OpenIDInnerConfig>,
}

#[derive(Deserialize, Debug)]
struct AADInnerConfig {
    pub tenant: String,
}

#[derive(Deserialize, Debug)]
struct OpenIDInnerConfig {
    pub discovery_url: String,
}

#[derive(Deserialize, Debug)]
struct OpenIDDiscoveryDocument {
    authorization_endpoint: String,
    jwks_uri: String,
    id_token_signing_alg_values_supported: Vec<String>,
    end_session_endpoint: Option<String>,
}

/// A JSON Web Key Set, as per the spec.
#[derive(Deserialize, Debug)]
struct JWKSet {
    pub keys: Vec<JWK>,
}

/// A JSON Web Key entry. We only extract the fields useful for this application.
#[derive(Deserialize, Debug)]
struct JWK {
    /// Key ID - see https://tools.ietf.org/html/rfc7517#section-4.5
    pub kid: String,
    /// X.509 Cert Chain - see https://tools.ietf.org/html/rfc7517#section-4.7
    pub x5c: Vec<String>,
}

#[derive(Debug)]
struct OpenIDCSRFSession {
    created: Instant,
    state_token: String,
    nonce_token: String,
}

#[derive(Debug)]
pub struct OpenIDAuthnBackend {
    redirect_url_base: String,
    logout_url_base: Option<String>,
    jwt_validator: jwt::Validation,
    jwks_keys: RwLock<HashMap<String, JWK>>, // kid (Key ID) -> JWK struct
    csrf_sessions: RwLock<HashMap<String, OpenIDCSRFSession>>, // Cookie token -> CSRF session metadata
    sub_email_map: RwLock<HashMap<String, String>>, // 'sub' OpenID field (Section 2, Core 1.0) -> email
}

impl OpenIDAuthnBackend {
    pub fn new(conf_loc: &str, hpas_conf: &HPASConfig) -> Arc<Self> {
        let conf: Config = {
            info!("Loading AAD/OpenID configuration from {}", conf_loc);
            let mut f = File::open(conf_loc).unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            toml::from_str(&contents)
        }.map_err(|e| {
            panic!("Unable to read AAD/OpenID configuration: {}", e);
        }).unwrap();

        let discovery = match hpas_conf.get_authn_provider().as_str() {
            "aad" => format!(
                "https://login.microsoftonline.com/{}/.well-known/openid-configuration",
                conf.aad.expect("AAD config not specified").tenant,
            ),
            "openid" => conf.openid.expect("OpenID config not specified").discovery_url,
            _ => unreachable!(),
        };

        info!("Attempting OpenID Connect discovery at: {}", discovery);
        let mut res = HTTP
            .get(&discovery)
            .send()
            .map_err(|e| panic!("Error fetching OpenID metadata: {}", e))
            .unwrap();
        if res.status() != reqwest::StatusCode::Ok {
            panic!("Unable to fetch OpenID metadata; server returned HTTP {}: {}",
                   res.status().as_u16(),
                   res.status().canonical_reason().unwrap_or("(unknown)"));
        }
        let meta: OpenIDDiscoveryDocument = res.json().expect("Unable to parse OpenID metadata response!");

        info!("Fetching JSON Web Key Set specified in OpenID metadata: {}", meta.jwks_uri);
        let mut jwks_res = HTTP
            .get(&meta.jwks_uri)
            .send()
            .map_err(|e| panic!("Error fetching OpenID metadata: {}", e))
            .unwrap();
        if jwks_res.status() != reqwest::StatusCode::Ok {
            panic!("Unable to fetch JSON Web Key Set; server returned HTTP {}: {}",
                   jwks_res.status().as_u16(),
                   jwks_res.status().canonical_reason().unwrap_or("(unknown)"));
        }
        let mut jwks: JWKSet = jwks_res.json().expect("Unable to parse JSON Web Key Set response!");
        if jwks.keys.len() == 0 {
            panic!("No acceptable JSON Web Keys in the JWT Set!")
        }

        let mut jwks_keys: HashMap<String, JWK> = HashMap::new();
        for jwk in jwks.keys.drain(..) {
            let k = jwk.kid.clone();
            jwks_keys.insert(k, jwk);
        }

        let mut allowed_algos: Vec<jwt::Algorithm> = Vec::new();
        for alg in meta.id_token_signing_alg_values_supported {
            match alg.as_str() {
                "RS256" => allowed_algos.push(jwt::Algorithm::RS256),
                "RS384" => allowed_algos.push(jwt::Algorithm::RS384),
                "RS512" => allowed_algos.push(jwt::Algorithm::RS512),
                h if h.starts_with("HS") =>
                    warn!("JWT algorithm '{}' in id_token_signing_alg_values_supported but HMACs are not supported", h),
                "none" =>
                    warn!("JWT algorithm 'none' in id_token_signing_alg_values_supported; ignoring."),
                other =>
                    warn!("Unknown or unacceptable JWT algorithm '{}'; ignoring.", other),
            }
        }
        if allowed_algos.len() == 0 {
            panic!("No acceptable algorithms in OpenID Connect metadata!");
        }

        unimplemented!()
    }
}

impl<'a> AuthnBackend for OpenIDAuthnBackend {
    fn get_rocket_routes(&self) -> Vec<Route> {
        routes![]
    }

    fn authenticate(&self, _username: &str, _password: &str) -> Result<String, AuthnFailure> {
        Err(AuthnFailure::NotSupported())
    }

    fn add_to_client_meta(&self, meta: &mut Value) {
        meta["openid_url"] = Value::String("/api/authn/oauth2".to_string());
    }
}
