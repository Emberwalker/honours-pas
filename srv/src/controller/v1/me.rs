v1_imports!();

use rocket::Route;

use db::student::{self, mark, selection, comment};
use db::project;

pub fn get_routes() -> Vec<Route> {
    routes![get_marks, add_mark, rm_mark]
}

#[get("/me/marks")]
fn get_marks(usr: student::Student, conn: DatabaseConnection) -> V1Response<MarkList> {
    let marks = mark::get_all_for_student(&conn, usr.id).map_err(select_error_handler!("no marks found"))?;
    Ok(Json(MarkList {
        projects: marks,
    }))
}

#[post("/me/marks", data = "<body>")]
fn add_mark(body: Json<MarkMessage>, usr: student::Student, conn: DatabaseConnection) -> V1Response<GenericMessage> {
    project::get_project(&conn, body.id).map_err(select_error_handler!("no such project"))?;
    mark::create(&conn, &mark::NewStudentMark {
        student: usr.id,
        project: body.id,
    }).map_err(|e| {
        use diesel::result::Error::DatabaseError;
        use diesel::result::DatabaseErrorKind;
        match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _) => ok!("mark already exists"),
            _ => diesel_error_handler!(e),
        }
    })?;
    Ok(generic_message!("ok"))
}

#[delete("/me/marks/<id>")]
fn rm_mark(id: i32, usr: student::Student, conn: DatabaseConnection) -> V1Response<GenericMessage> {
    mark::delete(&conn, &mark::StudentMark {
        student: usr.id,
        project: id,
    }).map_err(|e| diesel_error_handler!(e))?;
    Ok(generic_message!("ok"))
}