v1_imports!();

use std::sync::Arc;

use rocket::{Route, State};

use db::staff;
use db::student;
use session::SessionManager;
use authn::AuthnHolder;

pub fn get_routes() -> Vec<Route> {
    routes![get_students, get_curr_students, rm_student, new_students]
}

#[get("/students")]
fn get_students(
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> V1Response<StudentList> {
    match student::get_all(&conn) {
        Ok(v) => Ok(Json(StudentList {
            students: v,
        })),
        Err(e) => {
            error!("Unable to fetch students: {:?}", e);
            Err(internal_server_error!("database error"))
        }
    }
}

#[get("/students/current")]
fn get_curr_students(_usr: staff::Admin, conn: DatabaseConnection) -> V1Response<StudentList> {
    match student::get_all_current(&conn) {
        Ok(v) => Ok(Json(StudentList {
            students: v,
        })),
        Err(e) => {
            error!("Unable to fetch students: {:?}", e);
            Err(internal_server_error!("database error"))
        }
    }
}

#[delete("/students/<id>")]
fn rm_student(
    id: i32,
    _usr: staff::Admin,
    conn: DatabaseConnection,
    auth: State<AuthnHolder>,
    manager: State<Arc<SessionManager>>,
) -> V1Response<GenericMessage> {
    let target = student::get(&conn, id).map_err(|e| {
        match e {
            SelectError::NoSuchValue() => not_found!("no such student"),
            SelectError::DieselError(e) => {
                error!("Diesel error fetching student: {}", e);
                debug!("Additional information: {:?}", e);
                internal_server_error!("database error")
            }
        }
    })?;
    student::delete(&conn, &target).map_err(|e| {
        error!("Diesel error deleting student: {}", e);
        debug!("Additional information: {:?}", e);
        internal_server_error!("database error")
    })?;
    manager.remove_session(&target.email, &auth);
    Ok(generic_message!("ok"))
}

#[post("/students", data = "<body>")]
fn new_students(
    body: Json<NewStudentList>,
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> V1Response<GenericMessage> {
    student::create_batch(&conn, &body.students).map_err(|e| {
        error!("Diesel error creating students: {}", e);
        debug!("Additional information: {:?}", e);
        internal_server_error!("database error")
    })?;
    Ok(generic_message!("ok"))
}