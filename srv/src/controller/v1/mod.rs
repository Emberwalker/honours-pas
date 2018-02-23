use std::sync::Arc;

use rocket::{Route, State};
use rocket::response::status;
use rocket::http::{Cookies, Status};
use rocket_contrib::Json;

use config::Config as HPASConfig;
use db::{user, DatabaseConnection};
use authn::{AuthnBackend, AuthnFailure, AuthnHolder};
use session::{Session, SessionManager};

mod types;
use self::types::*;

pub fn get_routes(_conf: &HPASConfig) -> Vec<Route> {
    routes![login]
}

#[post("/auth", data = "<body>")]
fn login(
    body: Json<LoginMessage>,
    conn: DatabaseConnection,
    authn_manager: State<AuthnHolder>,
    session_manager: State<Arc<SessionManager>>,
    mut cookies: Cookies,
) -> Result<Json<GenericMessage>, status::Custom<Json<GenericMessage>>> {
    let res = match authn_manager.authenticate(&body.username, &body.password) {
        Ok(email) => email,
        Err(e) => match e {
            AuthnFailure::Error() => {
                return Err(status::Custom(
                    Status::InternalServerError,
                    Json(GenericMessage {
                        message: "internal server error".to_string(),
                    }),
                ))
            }
            _ => {
                return Err(status::Custom(
                    Status::Unauthorized,
                    Json(GenericMessage {
                        message: "incorrect username or password".to_string(),
                    }),
                ))
            }
        },
    };

    // Check this is actually a valid user here (not just in e.g. an AD Forest)
    match user::find_user(&conn, &res) {
        None => {
            return Err(status::Custom(
                Status::Unauthorized,
                Json(GenericMessage {
                    message: "user does not exist".to_string(),
                }),
            ))
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
