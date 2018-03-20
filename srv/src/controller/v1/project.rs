v1_imports!();

use rocket::Route;

use db::{project, staff, session, user};
use session::Session;

pub fn get_routes() -> Vec<Route> {
    routes![get_projs, new_proj, update_proj, rm_proj]
}

#[get("/projects")]
fn get_projs(conn: DatabaseConnection, session: Session) -> V1Response<ProjectList> {
    let res = match user::find_user(&conn, &session.email[..]) {
        Some(user::User::Staff(_s)) => project::get_all(&conn),
        Some(user::User::Student(_s)) => project::get_all_current(&conn),
        None => panic!("A session exists for a user which does not exist!"),
    }.map_err(select_error_handler!("no projects found"))?;

    let projs = project::attach_staff(&conn, res).map_err(select_error_handler!("error fetching staff"))?;
    
    Ok(Json(ProjectList {
        projects: projs,
    }))
}

#[post("/projects", data = "<body>")]
fn new_proj(
    mut body: Json<project::NewProjectWithStaff>,
    usr: staff::Staff,
    conn: DatabaseConnection,
) -> V1Response<project::ProjectWithStaff> {
    if !usr.is_admin {
        body.supervisor_name = usr.full_name;
        body.supervisor_email = usr.email;
    }

    match project::create_with_staff(&conn, &body) {
        Ok(p) => Ok(Json(p)),
        Err(e) => Err(diesel_error_handler!(e)),
    }
}

#[put("/projects/<id>", data = "<body>")]
fn update_proj(
    id: i32,
    body: Json<project::Project>,
    usr: staff::Staff,
    conn: DatabaseConnection,
) -> V1Response<project::Project> {
    if !usr.is_admin && usr.email != body.supervisor_email {
        return Err(bad_request!("you do not own that project"));
    }

    if body.id != id {
        return Err(bad_request!("project ID does not match ID in body"))
    }

    let current_proj = project::get_project(&conn, id).map_err(|e| {
        match e {
            SelectError::NoSuchValue() => not_found!("no such project"),
            SelectError::DieselError(e) => diesel_error_handler!(e),
        }
    })?;

    let (is_curr, _) = session::get_session(&conn, current_proj.session).map_err(|_e| {
        internal_server_error!("database error")
    })?;

    if !is_curr {
        return Err(bad_request!("cannot edit an archived project"));
    }

    project::update(&conn, &body).map_err(|e| diesel_error_handler!(e))?;

    Ok(body)
}

#[delete("/projects/<id>")]
fn rm_proj(id: i32, usr: staff::Admin, _conn: DatabaseConnection) -> V1Response<GenericMessage> {
    // TODO
    error!("Attempt to delete project {} from {}; not implemented.", id, usr.email);
    Err(not_implemented!("not implemented"))
}