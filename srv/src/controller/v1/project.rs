v1_imports!();

use rocket::Route;

use db::{user, DatabaseConnection, SelectError};
use db::{project, staff, session};
use session::Session;

pub fn get_routes() -> Vec<Route> {
    routes![get_projs, new_proj, update_proj, rm_proj]
}

#[get("/projects")]
fn get_projs(conn: DatabaseConnection, session: Session) -> Result<Json<ProjectList>, ErrorResponse> {
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
            SelectError::NoSuchValue() => Err(not_found!("no projects found")),
            SelectError::DieselError(e) => {
                error!("Diesel error fetching all projects: {}", e);
                Err(internal_server_error!("an error occured"))
            }
        }
    }
}

#[post("/projects", data = "<body>")]
fn new_proj(
    mut body: Json<project::NewProject>,
    usr: staff::Staff,
    conn: DatabaseConnection,
) -> Result<Json<project::Project>, ErrorResponse> {
    if !usr.is_admin {
        body.supervisor_name = usr.full_name;
        body.supervisor_email = usr.email;
    }

    match project::create(&conn, &body) {
        Ok(p) => Ok(Json(p)),
        Err(e) => {
            error!("Diesel error when creating project: {}", e);
            debug!("Additional information: {:?}", e);
            Err(internal_server_error!("database error"))
        }
    }
}

#[put("/projects/<id>", data = "<body>")]
fn update_proj(
    id: i32,
    body: Json<project::Project>,
    usr: staff::Staff,
    conn: DatabaseConnection,
) -> Result<Json<project::Project>, ErrorResponse> {
    if !usr.is_admin && usr.email != body.supervisor_email {
        return Err(bad_request!("you do not own that project"));
    }

    if body.id != id {
        return Err(bad_request!("project ID does not match ID in body"))
    }

    let current_proj = project::get_project(&conn, id).map_err(|e| {
        match e {
            SelectError::NoSuchValue() => not_found!("no such project"),
            SelectError::DieselError(e) => {
                error!("Diesel error fetching a project: {}", e);
                debug!("Additional information: {:?}", e);
                internal_server_error!("database error")
            },
        }
    })?;

    let (is_curr, _) = session::get_session(&conn, current_proj.session).map_err(|_e| {
        internal_server_error!("database error")
    })?;

    if !is_curr {
        return Err(bad_request!("cannot edit an archived project"));
    }

    project::update(&conn, &body).map_err(|e| {
        error!("Diesel error occured updating project: {}", e);
        debug!("Additional information: {:?}", e);
        internal_server_error!("database error")
    })?;

    Ok(body)
}

#[delete("/projects/<id>")]
fn rm_proj(
    id: i32,
    usr: staff::Admin,
    _conn: DatabaseConnection,
) -> Result<Json<GenericMessage>, ErrorResponse> {
    // TODO
    error!("Attempt to delete project {} from {}; not implemented.", id, usr.email);
    Err(not_implemented!("not implemented"))
}