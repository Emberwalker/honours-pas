pub use super::models::Student;
pub use super::models::new::Student as NewStudent;

generate_create_fn!(students, NewStudent, Student, id, i32);

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
