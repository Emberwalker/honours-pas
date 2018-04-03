v1_imports!();

use std::sync::Arc;

use rocket::{Route, State};

use db::{staff, student, session};
use session::SessionManager;
use authn::AuthnHolder;

pub fn get_routes() -> Vec<Route> {
    routes![get_students, get_curr_students, rm_student, new_students]
}

#[get("/students")]
fn get_students(_usr: staff::Admin, conn: DatabaseConnection) -> V1Response<StudentList> {
    match student::get_all(&conn) {
        Ok(v) => Ok(Json(StudentList { students: v })),
        Err(e) => {
            error!("Unable to fetch students: {:?}", e);
            Err(internal_server_error!("database error"))
        }
    }
}

#[get("/students/current")]
fn get_curr_students(_usr: staff::Admin, conn: DatabaseConnection) -> V1Response<StudentList> {
    match student::get_all_current(&conn) {
        Ok(v) => Ok(Json(StudentList { students: v })),
        Err(SelectError::NoSuchValue()) => Ok(Json(StudentList { students: Vec::new() })),
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
    let target = student::get(&conn, id).map_err(|e| match e {
        SelectError::NoSuchValue() => not_found!("no such student"),
        SelectError::DieselError(e) => diesel_error_handler!(e),
    })?;
    student::delete(&conn, &target).map_err(|e| diesel_error_handler!(e))?;
    manager.remove_session(&target.email, &auth);
    Ok(generic_message!("ok"))
}

#[post("/students", data = "<body>")]
fn new_students(
    mut body: Json<NewStudentList>,
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> V1Response<GenericMessage> {
    let sess = match session::get_latest_session(&conn) {
        Ok(s) => s,
        Err(SelectError::NoSuchValue()) => return Err(bad_request!("no current session")),
        Err(SelectError::DieselError(e)) => {
            error!("Unable to fetch latest session: {}", e);
            return Err(internal_server_error!("database error"));
        }
    };

    let students = body.students.drain(..)
        .filter(|s| s.email != "" && s.full_name != "")
        .map(move |s| student::NewStudent {
            email: s.email,
            full_name: s.full_name,
            last_session: sess.id,
        })
        .collect::<Vec<student::NewStudent>>();

    student::create_batch(&conn, &students).map_err(|e| diesel_error_handler!(e))?;
    Ok(generic_message!("ok"))
}
