use rocket::{Outcome, Request};
use rocket::request::{self, FromRequest};
use rocket::http::Status;

pub use super::models::Student;
pub use super::models::new::Student as NewStudent;

use super::{DatabaseConnection, SelectError};
use session::Session;

// Enable upsert on the email field.
generate_crud_fns!(students, NewStudent, Student, (email -> full_name, last_session));

pub fn find_email(conn: &DatabaseConnection, student_email: &str) -> Result<Student, SelectError> {
    generate_select_body!(single, conn, students, Student, (email, student_email))
}

/*pub fn find_all_by_session(
    conn: &DatabaseConnection,
    session: i32,
) -> Result<Vec<Student>, SelectError> {
    generate_select_body!(multi, conn, students, Student, (last_session, session))
}*/

impl<'a,'r> FromRequest<'a,'r> for Student {
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
    pub use super::super::models::StudentSelection;
    pub use super::super::models::new::StudentSelection as NewStudentSelection;

    generate_crud_fns!(student_selections, NewStudentSelection, StudentSelection, (student, project -> weight));
}

pub mod mark {
    pub use super::super::models::StudentMark;
    pub use super::super::models::new::StudentMark as NewStudentMark;

    generate_crud_fns!(student_marks, NewStudentMark, StudentMark, noupdate);
}

pub mod comment {
    pub use super::super::models::StudentComment;
    pub use super::super::models::new::StudentComment as NewStudentComment;

    generate_crud_fns!(student_comments, NewStudentComment, StudentComment);
}
