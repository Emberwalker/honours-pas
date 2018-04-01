use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::thread;

use url;
use rocket::{Route, State};
use rocket::http::{Cookies, Status};
use rocket::request::LenientForm;
use rocket::response::{status, content};
use serde_json::Value;
use toml;
use reqwest;
use jsonwebtoken as jwt;
use base64;
use openssl::x509;

use util;
use session::SessionManager;
use db::{user, DatabaseConnection};
use config::Config as HPASConfig;
use super::{AuthnBackend, AuthnFailure, AuthnHolder};

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
        })
            .unwrap();

        match hpas_conf.get_authn_provider().as_str() {
            "aad" => ConfigWrapper::AAD(conf.aad.expect("No Azure AD confiuration specified!")),
            "openid" => {
                ConfigWrapper::OpenID(conf.openid.expect("No OpenID configuration specified!"))
            }
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

struct ParsedJWK {
    pub kid: String,
    pub key: Vec<u8>,
}

// OpenID responses. See:
// https://docs.microsoft.com/en-us/azure/active-directory/develop/active-directory-protocols-openid-connect-code#sample-response
#[derive(FromForm, Deserialize, Debug)]
struct OpenIDSuccess {
    pub id_token: String,
    pub state: String,
}

#[derive(FromForm, Deserialize, Debug)]
struct OpenIDError {
    pub error: String,
    pub error_description: String,
}

/// OpenID claims relevant to this application.
#[derive(Deserialize, Debug)]
struct OpenIDClaims {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub nonce: String,
}

#[derive(Clone, Debug)]
struct OpenIDCSRFSession {
    created: Instant,
    state_token: String,
    nonce_token: String,
}

pub struct OpenIDAuthnBackend {
    redirect_url_base: url::Url,
    jwks_update_url: url::Url,
    server_url_base: url::Url,
    logout_url_base: Option<url::Url>,
    client_id: Option<String>,
    jwt_validator: jwt::Validation,
    jwks_keys: RwLock<HashMap<String, ParsedJWK>>, // kid (Key ID) -> JWK struct
    csrf_sessions: RwLock<HashMap<String, OpenIDCSRFSession>>, // Cookie token -> CSRF session metadata
    old_tokens: RwLock<HashMap<String, String>>, // Email -> old id_token (for logout id_token_hint)
}

impl OpenIDAuthnBackend {
    pub fn new(conf_loc: &str, hpas_conf: &HPASConfig) -> Arc<Self> {
        let conf = ConfigWrapper::new(conf_loc, hpas_conf);

        let mut res = http_fetch(&conf.get_discovery_uri(), "OpenID metadata");
        let meta: OpenIDDiscoveryDocument = res.json()
            .expect("Unable to parse OpenID metadata response!");
        let jwks_keys =
            jwks_fetch(&meta.jwks_uri).expect("Unable to load initial JSON Web Key Set");

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
            redirect_url_base: url::Url::parse(&meta.authorization_endpoint).unwrap(),
            jwks_update_url: url::Url::parse(&meta.jwks_uri).unwrap(),
            server_url_base: url::Url::parse(&hpas_conf.get_server_address())
                .expect("Invalid server address!"),
            logout_url_base: meta.end_session_endpoint
                .map(|it| url::Url::parse(&it).unwrap()),
            client_id: conf.get_audience(),
            jwt_validator: validation,
            jwks_keys: RwLock::new(jwks_keys),
            csrf_sessions: RwLock::new(HashMap::new()),
            old_tokens: RwLock::new(HashMap::new()),
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
                        match jwks_fetch(backend.jwks_update_url.as_str()) {
                            Ok(set) => {
                                let mut keys = backend.jwks_keys.write().unwrap();
                                *keys = set;
                                info!("JSON Web Key Set updated successfully.");
                            }
                            Err(_) => {
                                warn!(
                                    "Unable to perform regular JWK set update. Using old values."
                                );
                            }
                        }
                    }

                    // Bump iteration count
                    iter_count += 1;
                }
            })
            .expect("maintenance thread creation");
    }

    fn new_csrf_session(&self) -> OpenIDCSRFSession {
        let state = util::generate_rand_string(32);
        let nonce = util::generate_rand_string(32);

        let session = OpenIDCSRFSession {
            created: Instant::now(),
            state_token: state.clone(),
            nonce_token: nonce.clone(),
        };

        let ret_session = session.clone();

        {
            let mut sessions = self.csrf_sessions.write().unwrap();
            sessions.insert(state.clone(), session);
        }

        ret_session
    }

    fn pull_csrf_session(&self, state: &str) -> Result<OpenIDCSRFSession, ()> {
        let mut sessions = self.csrf_sessions.write().unwrap();
        sessions.remove(state).ok_or(())
    }
}

impl<'a> AuthnBackend for OpenIDAuthnBackend {
    fn get_rocket_routes(&self) -> Vec<Route> {
        routes![get_redirect, post_success, post_error]
    }

    fn authenticate(&self, _username: &str, _password: &str) -> Result<String, AuthnFailure> {
        Err(AuthnFailure::NotSupported())
    }

    fn add_to_client_meta(&self, meta: &mut Value) {
        meta["openid_url"] = Value::String("/api/authn/openid".to_string());
    }

    fn on_logout(&self, email: &str) -> Option<String> {
        let old_token = {
            let mut token_bag = self.old_tokens.write().unwrap();
            token_bag.remove(email)
        };
        match self.logout_url_base {
            Some(ref url) => {
                let mut url = url.clone();
                if let Some(ref token) = old_token {
                    url.query_pairs_mut().append_pair("id_token_hint", token);
                }
                Some(url.to_string())
            },
            None => None,
        }
    }
}

fn http_fetch(url: &str, type_name: &str) -> reqwest::Response {
    info!("Fetching {} from: {}", type_name, url);
    let res = HTTP.get(url)
        .send()
        .map_err(|e| panic!("Error fetching {}: {}", type_name, e))
        .unwrap();
    if res.status() != reqwest::StatusCode::Ok {
        panic!(
            "Unable to fetch {}; server returned HTTP {}: {}",
            type_name,
            res.status().as_u16(),
            res.status().canonical_reason().unwrap_or("(unknown)")
        );
    }
    res
}

fn jwks_fetch(url: &str) -> Result<HashMap<String, ParsedJWK>, ()> {
    let mut jwks_res = http_fetch(url, "JSON Web Key Set");
    let mut jwks: JWKSet = jwks_res.json().map_err(|e| {
        error!("Unable to parse JWKS response from '{}': {}", url, e);
        ()
    })?;
    if jwks.keys.len() == 0 {
        error!("No acceptable JSON Web Keys in the JWT Set at '{}'!", url);
        return Err(());
    }

    let mut jwks_keys: HashMap<String, ParsedJWK> = HashMap::new();
    for jwk in jwks.keys.drain(..) {
        let k = jwk.kid.clone();
        // We ignore the return from this, since we insert from in the chain.
        let _ = jwk.x5c.get(0).ok_or("No certs in x5c chain!".to_string())
            .and_then(|raw| base64::decode(&raw).map_err(|e| e.to_string()))
            .and_then(|b64| x509::X509::from_der(&b64).map_err(|e| e.to_string()))
            .and_then(|crt| crt.public_key().map_err(|e| e.to_string()))
            .and_then(|pk| pk.rsa().map_err(|e| e.to_string()))
            .and_then(|pk| pk.public_key_to_der_pkcs1().map_err(|e| e.to_string()))
            .and_then(|bytes| {
                jwks_keys.insert(k, ParsedJWK {
                    kid: jwk.kid,
                    key: bytes,
                });
                Ok(())
            })
            .map_err(|e| warn!("Error parsing x5c chain: {}", e));
    }

    Ok(jwks_keys)
}

#[inline]
fn decode(backend: &OpenIDAuthnBackend, token: &str) -> Result<OpenIDClaims, ()> {
    let header = jwt::decode_header(&token).map_err(|e| {
        warn!("Error parsing JWT header: {}", e);
        ()
    })?;
    match header.kid {
        Some(ref kid) => decode_with_kid(backend, token, kid), // TODO
        None => decode_without_kid(backend, token), // TODO
    }
}

#[inline]
fn decode_with_kid(backend: &OpenIDAuthnBackend, token: &str, kid: &str) -> Result<OpenIDClaims, ()> {
    let key: Vec<u8>;
    {
        let keys = backend.jwks_keys.read().unwrap();
        let jwk = keys.get(kid).ok_or(())?;
        key = jwk.key.clone();
    }

    Ok(jwt::decode::<OpenIDClaims>(token, &key, &backend.jwt_validator).map_err(|_| ())?.claims)
}

#[inline]
fn decode_without_kid(backend: &OpenIDAuthnBackend, token: &str) -> Result<OpenIDClaims, ()> {
    for ref k in backend.jwks_keys.read().unwrap().keys() {
        let res = decode_with_kid(backend, token, k);
        if let Ok(claims) = res {
            return Ok(claims);
        }
    }
    Err(())
}

// Helper to generate more informative error pages (since these can be served to humans directly)
#[inline]
fn get_failure(stat: Status, body: &str) -> status::Custom<content::Html<String>> {
    status::Custom(stat, content::Html(format!(r#"
        <head>
            <title>Honours Project Allocation System</title>
        </head>
        <body>
            <h1>Login Error</h1>
            <p>
                An error occurred when logging you in. Please try again. If this keeps occuring, ask your system
                administrator for assistance. Additional information may be available below. Click <a href="/">here</a>
                to return to the home page and retry.
            </p>
            <p><i>{}</i></p>
        </body>
    "#, body)))
}

#[get("/openid")]
fn get_redirect(auth: State<AuthnHolder>) -> util::RedirectWithBody {
    let auth = (*auth.inner())
        .0
        .downcast_ref::<OpenIDAuthnBackend>()
        .expect("Downcast to OpenID provider");
    let session = auth.new_csrf_session();

    let mut redirect = auth.redirect_url_base.clone();
    let mut redir_uri = auth.server_url_base.clone();
    redir_uri.set_path("api/authn/openid");

    redirect
        .query_pairs_mut()
        .append_pair("scope", "openid")
        .append_pair("response_type", "id_token")
        .append_pair("nonce", &session.nonce_token)
        .append_pair("redirect_uri", redir_uri.as_str())
        .append_pair("response_mode", "form_post")
        .append_pair("state", &session.state_token);

    if let Some(ref id) = auth.client_id {
        // Required for Azure AD. Not sure about others.
        redirect.query_pairs_mut().append_pair("client_id", id);
    }

    util::RedirectWithBody::to(redirect.as_str())
}

#[post("/openid", data = "<res>")]
fn post_success(
    res: LenientForm<OpenIDSuccess>,
    auth: State<AuthnHolder>,
    session_manager: State<Arc<SessionManager>>,
    conn: DatabaseConnection,
    mut cookies: Cookies,
) -> Result<util::RedirectWithBody, status::Custom<content::Html<String>>> {
    let auth = (*auth.inner())
        .0
        .downcast_ref::<OpenIDAuthnBackend>()
        .expect("Downcast to OpenID provider");

    let response = res.into_inner();

    let csrf_session = auth.pull_csrf_session(&response.state).map_err(|_| get_failure(
        Status::BadRequest,
        "No matching CSRF session found. Please try logging in again."
    ))?;

    // Decode and verify JSON Web Token
    let decoded = decode(auth, &response.id_token).map_err(|_|
        get_failure(Status::BadRequest, "Unable to decode or verify token."))?;

    // Check nonce & state - this is required by the Azure documentation for security purposes.
    if csrf_session.nonce_token != decoded.nonce || csrf_session.state_token != response.state {
        return Err(get_failure(Status::BadRequest, "Nonce or state mismatch (CSRF violation)."));
    }

    // Check this is actually a valid user here (not just in e.g. an AD Forest)
    let _usr = match user::find_user(&conn, &decoded.email) {
        None => {
            return Err(get_failure(Status::Forbidden, "Your chosen credentials are not valid for this system."));
        }
        Some(u) => {
            debug!("User login: {:?}", u);
            u
        }
    };

    {
        let mut token_bag = auth.old_tokens.write().unwrap();
        token_bag.insert(decoded.email.clone(), response.id_token.clone());
    }

    debug!(
        "New session: {:?}",
        session_manager.new_session(&decoded.email, &mut cookies)
    );

    Ok(util::RedirectWithBody::to("/"))
}

#[post("/openid", data = "<res>", rank = 2)]
fn post_error(res: LenientForm<OpenIDError>) -> status::Custom<content::Html<String>> {
    let response = res.into_inner();
    get_failure(Status::Forbidden, &format!(
        "Authentication error: {} ({})",
        &response.error_description,
        &response.error
    ))
}