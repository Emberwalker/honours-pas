use std::sync::Arc;

use rocket::{Catcher, Route, State};
use rocket::http::Cookies;

use config::Config as HPASConfig;
use db::user;
use authn::{AuthnBackend, AuthnFailure, AuthnHolder};
use session::SessionManager;

mod types;

#[macro_use]
mod macros;
mod errors;
mod session;
mod project;
mod staff;
mod student;
mod me;
mod meta;

v1_imports!();

pub fn get_routes(conf: &HPASConfig) -> Vec<Route> {
    // Disable login route for OpenID/Azure AD provider.
    let mut mod_routes =
        if conf.get_authn_provider() == "openid" || conf.get_authn_provider() == "aad" {
            routes![whoami]
        } else {
            routes![login, whoami]
        };

    concat_vec![
        mod_routes,
        session::get_routes(),
        project::get_routes(),
        staff::get_routes(),
        student::get_routes(),
        me::get_routes(),
        meta::get_routes(),
    ]
}

pub fn get_catchers(_conf: &HPASConfig) -> Vec<Catcher> {
    errors::get_catchers()
}

#[post("/auth", data = "<body>")]
fn login(
    body: Json<LoginMessage>,
    conn: DatabaseConnection,
    authn_manager: State<AuthnHolder>,
    session_manager: State<Arc<SessionManager>>,
    mut cookies: Cookies,
) -> V1Response<WhoAmIMessage> {
    let res = match authn_manager.authenticate(&body.username, &body.password) {
        Ok(email) => email,
        Err(e) => match e {
            AuthnFailure::Error() => {
                return Err(internal_server_error!("internal server error"));
            }
            _ => {
                return Err(forbidden!("incorrect username or password"));
            }
        },
    };

    // Check this is actually a valid user here (not just in e.g. an AD Forest)
    let usr = match user::find_user(&conn, &res) {
        None => {
            return Err(unauthorized!("user does not exist"));
        }
        Some(u) => {
            debug!("User login: {:?}", u);
            u
        }
    };

    debug!(
        "New session: {:?}",
        session_manager.new_session(&res, &mut cookies)
    );

    let resp = match usr {
        // TODO: Check student is valid for *this* session.
        user::User::Student(s) => WhoAmIMessage {
            email: s.email,
            name: s.full_name,
            user_type: "student".to_string(),
        },
        user::User::Staff(s) => WhoAmIMessage {
            email: s.email,
            name: s.full_name,
            user_type: match s.is_admin {
                true => "admin".to_string(),
                false => "staff".to_string(),
            },
        },
    };

    Ok(Json(resp))
}

#[get("/whoami")]
fn whoami(usr: user::User) -> Json<WhoAmIMessage> {
    let utype = match usr {
        user::User::Staff(ref s) => match s.is_admin {
            true => "admin",
            false => "staff",
        },
        user::User::Student(ref _s) => "student",
    };

    Json(WhoAmIMessage {
        email: usr.email(),
        name: usr.full_name(),
        user_type: utype.to_string(),
    })
}
