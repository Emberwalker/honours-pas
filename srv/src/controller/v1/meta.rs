v1_imports!();

use rocket::{Route, State};

use config::Config;
use authn::{AuthnBackend, AuthnHolder};

pub fn get_routes() -> Vec<Route> {
    routes![get_meta]
}

#[get("/meta")]
pub fn get_meta(auth: State<AuthnHolder>, conf: State<Config>) -> Result<String, ErrorResponse> {
    let authn_prov = conf.get_authn_provider();
    let mut v = json!({
        "auth": authn_prov
    });
    auth.add_to_client_meta(&mut v);
    Ok(v.to_string())
}