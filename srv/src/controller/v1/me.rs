v1_imports!();

use rocket::Route;
use num_traits::cast::FromPrimitive;
use bigdecimal::BigDecimal;

use db::student::{comment, mark, selection, Student};
use db::{project, session};

pub fn get_routes() -> Vec<Route> {
    routes![get_marks, add_mark, rm_mark, set_selections, set_comment]
}

#[get("/me/marks")]
fn get_marks(usr: Student, conn: DatabaseConnection) -> V1Response<MarkList> {
    let marks =
        mark::get_all_for_student(&conn, usr.id).map_err(select_error_handler!("no marks found"))?;
    Ok(Json(MarkList { projects: marks }))
}

#[post("/me/marks", data = "<body>")]
fn add_mark(
    body: Json<MarkMessage>,
    usr: Student,
    conn: DatabaseConnection,
) -> V1Response<GenericMessage> {
    project::get_project(&conn, body.id).map_err(select_error_handler!("no such project"))?;
    mark::create(
        &conn,
        &mark::NewStudentMark {
            student: usr.id,
            project: body.id,
        },
    ).map_err(|e| {
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
fn rm_mark(id: i32, usr: Student, conn: DatabaseConnection) -> V1Response<GenericMessage> {
    mark::delete(
        &conn,
        &mark::StudentMark {
            student: usr.id,
            project: id,
        },
    ).map_err(|e| diesel_error_handler!(e))?;
    Ok(generic_message!("ok"))
}

#[put("/me/selections", data = "<body>")]
fn set_selections(
    body: Json<SelectionList>,
    usr: Student,
    conn: DatabaseConnection,
) -> V1Response<GenericMessage> {
    use diesel::result;

    if body.selections.len() != 3 {
        return Err(bad_request!("only three selections are allowed"));
    }

    selection::clear_all_for_student(&conn, usr.id).map_err(|e| diesel_error_handler!(e))?;
    let raw_sels = &body.selections;
    let new_sels: Vec<selection::NewStudentSelection> = raw_sels
        .into_iter()
        .map(|it| selection::NewStudentSelection {
            student: usr.id,
            project: it.project,
            weight: BigDecimal::from_f64(it.weight).unwrap(),
        })
        .collect();

    selection::create_batch(&conn, &new_sels).map_err(|e| match e {
        result::Error::DatabaseError(result::DatabaseErrorKind::ForeignKeyViolation, _) => {
            bad_request!("unknown project in selections")
        }
        _ => diesel_error_handler!(e),
    })?;

    Ok(generic_message!("ok"))
}

#[put("/me/comment", data = "<body>")]
fn set_comment(
    body: Json<CommentMessage>,
    usr: Student,
    conn: DatabaseConnection,
) -> V1Response<GenericMessage> {
    let sess = session::get_latest_session(&conn)
        .map_err(select_error_handler!("unable to get current session"))?;
    comment::create(
        &conn,
        &comment::NewStudentComment {
            student: usr.id,
            session: sess.id,
            comment: body.comment.clone(),
        },
    ).map_err(|e| diesel_error_handler!(e))?;
    Ok(generic_message!("ok"))
}
