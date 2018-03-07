use rocket::Route;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;

use db::{user, DatabaseConnection, SelectError};
use db::{project, staff};
use session::Session;

use super::types::*;

pub fn get_routes() -> Vec<Route> {
    routes![get_projs, new_proj, rm_proj]
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

#[post("/projects", data = "<body>")]
fn new_proj(
    mut body: Json<project::NewProject>,
    usr: staff::Staff,
    conn: DatabaseConnection,
) -> Result<Json<project::Project>, status::Custom<Json<GenericMessage>>> {
    if !usr.is_admin {
        body.supervisor_name = usr.full_name;
        body.supervisor_email = usr.email;
    }

    match project::create(&conn, &body) {
        Ok(p) => Ok(Json(p)),
        Err(e) => {
            error!("Diesel error when creating project: {}", e);
            debug!("Additional information: {:?}", e);
            Err(status::Custom(Status::InternalServerError, Json(GenericMessage {
                message: "database error".to_string(),
            })))
        }
    }
}

#[delete("/projects/<id>")]
fn rm_proj(
    id: i32,
    usr: staff::Admin,
    _conn: DatabaseConnection,
) -> Result<Json<GenericMessage>, status::Custom<Json<GenericMessage>>> {
    // TODO
    error!("Attempt to delete project {} from {}; not implemented.", id, usr.email);
    Err(status::Custom(Status::NotImplemented, Json(GenericMessage {
        message: "not implemented".to_string(),
    })))
}