v1_imports!();

use rocket::{Route, State};

use authn::{AuthnBackend, AuthnHolder};
use config::Config;

pub fn get_routes() -> Vec<Route> {
    routes![get_meta]
}

#[allow(needless_pass_by_value)]
#[get("/meta")]
pub fn get_meta(auth: State<AuthnHolder>, conf: State<Config>) -> Result<String, ErrorResponse> {
    let authn_prov = conf.get_authn_provider();
    let mut v = if authn_prov == "aad" {
        // Convert Azure AD to OpenID (since the frontend has no concept of Azure AD)
        json!({ "auth": "openid" })
    } else {
        json!({ "auth": authn_prov })
    };
    auth.add_to_client_meta(&mut v);
    Ok(v.to_string())
}
