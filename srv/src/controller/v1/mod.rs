use std::sync::Arc;

use rocket::http::{Cookie, Cookies};
use rocket::response::content;
use rocket::{Catcher, Route, State};

use authn::{AuthnBackend, AuthnFailure, AuthnHolder};
use config::Config as HPASConfig;
use db::user;
use session::{Session, SessionManager};
use util;

mod types;

#[macro_use]
mod macros;
mod errors;
mod me;
mod meta;
mod project;
mod session;
mod staff;
mod student;

v1_imports!();

const LOGGED_OUT_HTML: &str = r#"
    <head>
        <title>Honours Project Allocation System</title>
    </head>
    <body>
        <h1>Logged out</h1>
        <p>You have been logged out. <a href="/">Click here</a> to return home and log back in.</p>
    </body>
"#;

pub fn get_routes(conf: &HPASConfig) -> Vec<Route> {
    // Disable login route for OpenID/Azure AD provider.
    let mut mod_routes =
        if conf.get_authn_provider() == "openid" || conf.get_authn_provider() == "aad" {
            routes![logout, logged_out, whoami]
        } else {
            routes![login, logout, logged_out, whoami]
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

#[allow(needless_pass_by_value)]
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
            user_type: if s.is_admin {
                "admin".to_string()
            } else {
                "staff".to_string()
            },
        },
    };

    Ok(Json(resp))
}

#[allow(needless_pass_by_value)]
#[get("/whoami")]
fn whoami(usr: user::User) -> Json<WhoAmIMessage> {
    let utype = match usr {
        user::User::Staff(ref s) => if s.is_admin {
            "admin"
        } else {
            "staff"
        },
        user::User::Student(ref _s) => "student",
    };

    Json(WhoAmIMessage {
        email: usr.email(),
        name: usr.full_name(),
        user_type: utype.to_string(),
    })
}

#[allow(needless_pass_by_value)]
#[get("/logout")]
fn logout(
    sess: Session,
    authn_manager: State<AuthnHolder>,
    session_manager: State<Arc<SessionManager>>,
    mut cookies: Cookies,
) -> Result<content::Html<&'static str>, util::RedirectWithBody> {
    cookies.remove_private(Cookie::named("session"));
    // Redirect if asked by the auth provider (e.g. Azure AD uses this to provide Single Sign Out)
    if let Some(ref redir) = session_manager.remove_session(&sess.email, &authn_manager) {
        return Err(util::RedirectWithBody::to(redir));
    }
    Ok(content::Html(LOGGED_OUT_HTML))
}

#[get("/logout", rank = 2)]
fn logged_out() -> content::Html<&'static str> {
    content::Html(LOGGED_OUT_HTML)
}
