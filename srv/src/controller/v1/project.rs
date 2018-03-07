use std::sync::Arc;

use rocket::{Route, State};
use rocket::response::status;
use rocket::http::{Cookies, Status};
use rocket_contrib::Json;

use config::Config as HPASConfig;
use db::{user, DatabaseConnection, SelectError};
use db::project;
use authn::{AuthnBackend, AuthnFailure, AuthnHolder};
use session::{Session, SessionManager};

use super::types::*;

pub fn get_routes() -> Vec<Route> {
    routes![get_projs]
}

#[get("/projects")]
fn get_projs(conn: DatabaseConnection, session: Session) -> Result<Json<ProjectList>, status::Custom<Json<GenericMessage>>> {
    let res = match user::find_user(&conn, &session.email[..]) {
        Some(user::User::Staff(_s)) => project::get_all(&conn),
        Some(user::User::Student(_s)) => project::get_all_current(&conn),
        None => panic!("A session exists for a user which does not exist!"),
    };
    
    if let Ok(projects) = res {
        Ok(Json(ProjectList {
            projects: projects,
        }))
    } else {
        match res.unwrap_err() {
            SelectError::NoSuchValue() => Err(status::Custom(
                Status::NotFound,
                Json(GenericMessage {
                    message: "no projects found".to_string(),
                })
            )),
            SelectError::DieselError(e) => {
                error!("Diesel error fetching all projects: {}", e);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(GenericMessage {
                        message: "an error occured".to_string(),
                    })
                ))
            }
        }
    }
}
