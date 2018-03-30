use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::thread;

use rocket::Route;
use serde_json::Value;
use toml;
use reqwest;
use jsonwebtoken as jwt;

use config::Config as HPASConfig;
use super::{AuthnBackend, AuthnFailure};

lazy_static! {
    static ref CSRF_DURATION: Duration = Duration::from_secs(5*60); // 5 minutes
    static ref CLEANUP_SLEEP: Duration = Duration::from_secs(30); // 30 seconds
    static ref HTTP: reqwest::Client = reqwest::Client::new();
}

const JWKS_REFRESH: u32 = 60; // 60 x 30s iterations = 30mins

#[derive(Debug)]
enum ConfigWrapper {
    AAD(AADInnerConfig),
    OpenID(OpenIDInnerConfig),
}

impl ConfigWrapper {
    pub fn new(conf_loc: &str, hpas_conf: &HPASConfig) -> Self {
        let conf: Config = {
            info!("Loading AAD/OpenID configuration from {}", conf_loc);
            let mut f = File::open(conf_loc).unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            toml::from_str(&contents)
        }.map_err(|e| {
            panic!("Unable to read AAD/OpenID configuration: {}", e);
        }).unwrap();

        match hpas_conf.get_authn_provider().as_str() {
            "aad" => ConfigWrapper::AAD(conf.aad.expect("No Azure AD confiuration specified!")),
            "openid" => ConfigWrapper::OpenID(conf.openid.expect("No OpenID configuration specified!")),
            _ => unreachable!(),
        }
    }

    pub fn get_audience(&self) -> Option<String> {
        match self {
            &ConfigWrapper::AAD(ref conf) => Some(conf.application_id.clone()),
            &ConfigWrapper::OpenID(ref conf) => conf.audience.clone(),
        }
    }

    pub fn get_discovery_uri(&self) -> String {
        match self {
            &ConfigWrapper::AAD(ref conf) => format!(
                    "https://login.microsoftonline.com/{}/.well-known/openid-configuration",
                    conf.tenant,
                ),
            &ConfigWrapper::OpenID(ref conf) => conf.discovery_url.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    pub aad: Option<AADInnerConfig>,
    pub openid: Option<OpenIDInnerConfig>,
}

#[derive(Deserialize, Debug)]
struct AADInnerConfig {
    pub tenant: String,
    pub application_id: String,
}

#[derive(Deserialize, Debug)]
struct OpenIDInnerConfig {
    pub discovery_url: String,
    pub audience: Option<String>,
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
    jwks_update_url: String,
    logout_url_base: Option<String>,
    jwt_validator: jwt::Validation,
    jwks_keys: RwLock<HashMap<String, JWK>>, // kid (Key ID) -> JWK struct
    csrf_sessions: RwLock<HashMap<String, OpenIDCSRFSession>>, // Cookie token -> CSRF session metadata
    sub_email_map: RwLock<HashMap<String, String>>, // 'sub' OpenID field (Section 2, Core 1.0) -> email
}

impl OpenIDAuthnBackend {
    pub fn new(conf_loc: &str, hpas_conf: &HPASConfig) -> Arc<Self> {
        let conf = ConfigWrapper::new(conf_loc, hpas_conf);

        let mut res = http_fetch(&conf.get_discovery_uri(), "OpenID metadata");
        let meta: OpenIDDiscoveryDocument = res.json().expect("Unable to parse OpenID metadata response!");
        let jwks_keys = jwks_fetch(&meta.jwks_uri).expect("Unable to load initial JSON Web Key Set");

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

        let mut validation = jwt::Validation {
            leeway: 60, // Allow up to 1min clock skew
            algorithms: allowed_algos,
            ..Default::default()
        };
        if let Some(aud) = conf.get_audience() {
            validation.set_audience(&aud);
        }
        debug!("{:?}", validation);

        let backend = Arc::new(OpenIDAuthnBackend {
            redirect_url_base: meta.authorization_endpoint,
            jwks_update_url: meta.jwks_uri,
            logout_url_base: meta.end_session_endpoint,
            jwt_validator: validation,
            jwks_keys: RwLock::new(jwks_keys),
            csrf_sessions: RwLock::new(HashMap::new()),
            sub_email_map: RwLock::new(HashMap::new()),
        });

        OpenIDAuthnBackend::start_cleanup_thread(Arc::clone(&backend));

        backend
    }

    /// Spawns a cleanup thread that maintains the provider state. Removes expired CSRF sessions and updates the JWKS
    /// used to verify messages from the OpenID provider.
    /// The behaviour of the thread is controlled by CLEANUP_SLEEP (time between executions), CSRF_DURATION (how long a
    /// CSRF session is valid for), and JWKS_REFRESH (executions between JWKS refreshes, which is fairly expensive).
    fn start_cleanup_thread(backend: Arc<OpenIDAuthnBackend>) {
        thread::Builder::new()
            .name("openid-maintenance".to_string())
            .spawn(move || {
                let mut iter_count = 1;
                let mut cleanup_vec: Vec<String> = Vec::new();
                loop {
                    thread::sleep(*CLEANUP_SLEEP);

                    // CSRF cleanup
                    // The extra blocks are to ensure locks are dropped as soon as possible; Rust will call the Drop
                    // trait on objects when they go out of scope, thus releasing the dropped locks.
                    let now = Instant::now();
                    {
                        let sessions = backend.csrf_sessions.read().unwrap();
                        for (k, v) in sessions.iter() {
                            if now.duration_since(v.created) > *CSRF_DURATION {
                                cleanup_vec.push(k.clone());
                            }
                        }
                    }
                    if !cleanup_vec.is_empty() {
                        info!("Purging {} expired login CSRF sessions.", cleanup_vec.len());
                        let mut sessions = backend.csrf_sessions.write().unwrap();
                        for k in cleanup_vec.drain(..) {
                            sessions.remove(&k);
                        }
                    }

                    // JWKS refresh
                    // This only runs every so often, so we only execute every JWKS_REFRESH iterations.
                    if iter_count >= JWKS_REFRESH {
                        iter_count = 0;
                        match jwks_fetch(&backend.jwks_update_url) {
                            Ok(set) => {
                                let mut keys = backend.jwks_keys.write().unwrap();
                                *keys = set;
                                info!("JSON Web Key Set updated successfully.");
                            },
                            Err(_) => {
                                warn!("Unable to perform regular JWK set update. Using old values.");
                            }
                        }
                    }

                    // Bump iteration count
                    iter_count += 1;
                }
            })
            .expect("maintenance thread creation");
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

fn http_fetch(url: &str, type_name: &str) -> reqwest::Response {
    info!("Fetching {} from: {}", type_name, url);
    let res = HTTP
            .get(url)
            .send()
            .map_err(|e| panic!("Error fetching {}: {}", type_name, e))
            .unwrap();
    if res.status() != reqwest::StatusCode::Ok {
        panic!("Unable to fetch {}; server returned HTTP {}: {}",
                type_name,
                res.status().as_u16(),
                res.status().canonical_reason().unwrap_or("(unknown)"));
    }
    res
}

fn jwks_fetch(url: &str) -> Result<HashMap<String, JWK>, ()> {
    let mut jwks_res = http_fetch(url, "JSON Web Key Set");
    let mut jwks: JWKSet = jwks_res.json().map_err(|e| {
        error!("Unable to parse JWKS response from '{}': {}", url, e);
        ()
    })?;
    if jwks.keys.len() == 0 {
        error!("No acceptable JSON Web Keys in the JWT Set at '{}'!", url);
        return Err(());
    }

    let mut jwks_keys: HashMap<String, JWK> = HashMap::new();
    for jwk in jwks.keys.drain(..) {
        let k = jwk.kid.clone();
        jwks_keys.insert(k, jwk);
    }

    Ok(jwks_keys)
}