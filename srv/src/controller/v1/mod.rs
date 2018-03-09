use std::sync::Arc;

use rocket::{Route, State};
use rocket::http::Cookies;

use config::Config as HPASConfig;
use db::{user, DatabaseConnection};
use authn::{AuthnBackend, AuthnFailure, AuthnHolder};
use session::{Session, SessionManager};

mod types;
use self::types::*;

#[macro_use]
mod macros;
mod project;
mod staff;

v1_imports!();

pub fn get_routes(_conf: &HPASConfig) -> Vec<Route> {
    concat_vec![
        routes![login, whoami],
        project::get_routes(),
        staff::get_routes(),
    ]
}

#[post("/auth", data = "<body>")]
fn login(
    body: Json<LoginMessage>,
    conn: DatabaseConnection,
    authn_manager: State<AuthnHolder>,
    session_manager: State<Arc<SessionManager>>,
    mut cookies: Cookies,
) -> Result<Json<GenericMessage>, ErrorResponse> {
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
    match user::find_user(&conn, &res) {
        None => {
            return Err(unauthorized!("user does not exist"));
        }
        Some(u) => debug!("User login: {:?}", u),
    };

    debug!(
        "New session: {:?}",
        session_manager.new_session(&res[..], &mut cookies)
    );

    Ok(Json(GenericMessage {
        message: "ok".to_string(),
    }))
}

#[get("/whoami")]
fn whoami(conn: DatabaseConnection, session: Session) -> Json<WhoAmIMessage> {
    let utype = match user::find_user(&conn, &session.email[..]) {
        Some(user::User::Staff(s)) => match s.is_admin {
            true => "admin",
            false => "staff",
        },
        Some(user::User::Student(_s)) => "student",
        None => panic!("A session exists for a user which does not exist!"),
    };

    Json(WhoAmIMessage {
        email: session.email,
        user_type: utype.to_string(),
    })
}