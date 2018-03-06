pub use super::models::Student;
pub use super::models::new::Student as NewStudent;

use super::{DatabaseConnection, SelectError};

generate_create_fn!(students, NewStudent, Student, id, i32);

pub fn find_email(conn: &DatabaseConnection, student_email: &str) -> Result<Student, SelectError> {
    generate_select_body!(single, conn, students, Student, (email, student_email))
}

pub fn find_all_by_session(
    conn: &DatabaseConnection,
    session: i32,
) -> Result<Vec<Student>, SelectError> {
    generate_select_body!(multi, conn, students, Student, (last_session, session))
}

pub mod selection {
    pub use super::super::models::StudentSelection;
    pub use super::super::models::new::StudentSelection as NewStudentSelection;

    generate_create_fn!(student_selections, NewStudentSelection, StudentSelection);
}

pub mod mark {
    pub use super::super::models::StudentMark;
    pub use super::super::models::new::StudentMark as NewStudentMark;

    generate_create_fn!(student_marks, NewStudentMark, StudentMark);
}

pub mod comment {
    pub use super::super::models::StudentComment;
    pub use super::super::models::new::StudentComment as NewStudentComment;

    generate_create_fn!(student_comments, NewStudentComment, StudentComment);
}
