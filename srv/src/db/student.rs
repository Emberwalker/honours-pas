use rocket::{Outcome, Request};
use rocket::request::{self, FromRequest};
use rocket::http::Status;

pub use super::models::Student;
pub use super::models::new::Student as NewStudent;
use super::session;

use super::{DatabaseConnection, SelectError};
use session::Session;

// Enable upsert on the email field.
generate_crud_fns!(students, NewStudent, Student, (email -> full_name, last_session));

pub fn get(conn: &DatabaseConnection, id: i32) -> Result<Student, SelectError> {
    generate_select_body!(single, conn, students, Student, (id, id))
}

pub fn find_email(conn: &DatabaseConnection, student_email: &str) -> Result<Student, SelectError> {
    generate_select_body!(single, conn, students, Student, (email, student_email))
}

pub fn get_all_by_session(
    conn: &DatabaseConnection,
    session: i32,
) -> Result<Vec<Student>, SelectError> {
    generate_select_body!(multi, conn, students, Student, (last_session, session))
}

pub fn get_all_current(conn: &DatabaseConnection) -> Result<Vec<Student>, SelectError> {
    let sess = session::get_latest_session(conn)?;
    get_all_by_session(conn, sess.id)
}

pub fn get_all(conn: &DatabaseConnection) -> Result<Vec<Student>, SelectError> {
    generate_select_body!(multi, conn, students, Student)
}

impl<'a, 'r> FromRequest<'a, 'r> for Student {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Student, ()> {
        let sess = request.guard::<Session>()?;
        let conn = request.guard::<DatabaseConnection>()?;

        match find_email(&conn, &sess.email) {
            Ok(s) => Outcome::Success(s),
            Err(SelectError::NoSuchValue()) => Outcome::Failure((Status::Forbidden, ())),
            Err(SelectError::DieselError(e)) => {
                error!("Diesel error fetching Student record: {}", e);
                debug!("Detailed error: {:?}", e);
                Outcome::Failure((Status::InternalServerError, ()))
            }
        }
    }
}

pub mod selection {
    use diesel::result::Error;
    use bigdecimal::BigDecimal;

    pub use super::super::models::StudentSelection;
    pub use super::super::models::new::StudentSelection as NewStudentSelection;
    use super::super::{DatabaseConnection, SelectError};
    use db::student;

    generate_crud_fns!(student_selections, NewStudentSelection, StudentSelection, (student, project -> weight));

    pub fn _get_all_for_student(
        conn: &DatabaseConnection,
        id: i32,
    ) -> Result<Vec<(i32, BigDecimal)>, SelectError> {
        let vals = generate_select_body!(
            multi,
            conn,
            student_selections,
            StudentSelection,
            (student, id)
        )?;
        Ok(vals.into_iter().map(|it| (it.project, it.weight)).collect())
    }

    pub fn clear_all_for_student(conn: &DatabaseConnection, id: i32) -> Result<(), Error> {
        use diesel;
        use diesel::prelude::*;
        use schema::student_selections;
        diesel::delete(student_selections::table.filter(student_selections::student.eq(id)))
            .execute(conn.raw())?;
        Ok(())
    }

    pub fn get_students_for_project(
        conn: &DatabaseConnection,
        proj: i32,
    ) -> Result<Vec<student::Student>, SelectError> {
        use diesel::prelude::*;
        use schema::{student_selections, students};

        let students = student_selections::table
            .inner_join(students::table)
            .filter(student_selections::project.eq(proj))
            .select(students::table::all_columns())
            .load::<student::Student>(conn.raw())
            .map_err(|e| match e {
                diesel::result::Error::NotFound => SelectError::NoSuchValue(),
                e => SelectError::DieselError(e),
            })?;

        Ok(students)
    }

    pub fn get_all_for_session(conn: &DatabaseConnection, sess: i32) -> Result<Vec<StudentSelection>, SelectError> {
        use diesel::prelude::*;
        use schema::{student_selections, students};

        let sels = student_selections::table
            .inner_join(students::table)
            .filter(students::last_session.eq(sess))
            .select(student_selections::table::all_columns())
            .load::<StudentSelection>(conn.raw())
            .map_err(|e| match e {
                diesel::result::Error::NotFound => SelectError::NoSuchValue(),
                e => SelectError::DieselError(e),
            })?;

        Ok(sels)
    }
}

pub mod mark {
    pub use super::super::models::StudentMark;
    pub use super::super::models::new::StudentMark as NewStudentMark;
    use super::super::{DatabaseConnection, SelectError};

    generate_crud_fns!(student_marks, NewStudentMark, StudentMark, noupdate);

    pub fn get_all_for_student(
        conn: &DatabaseConnection,
        id: i32,
    ) -> Result<Vec<i32>, SelectError> {
        let vals = generate_select_body!(multi, conn, student_marks, StudentMark, (student, id))?;
        Ok(vals.into_iter().map(|it| it.project).collect())
    }
}

pub mod comment {
    pub use super::super::models::StudentComment;
    pub use super::super::models::new::StudentComment as NewStudentComment;
    use super::super::{DatabaseConnection, SelectError};
    use db::session;

    generate_crud_fns!(student_comments, NewStudentComment, StudentComment, (student, session -> comment));

    pub fn _get_current_for_student(
        conn: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<String>, SelectError> {
        let s = session::get_latest_session(conn)?.id;
        let comm = generate_select_body!(
            single,
            conn,
            student_comments,
            StudentComment,
            (student, id),
            (session, s)
        )?;
        Ok(comm.comment)
    }
}
