pub use super::models::Student;
use super::models::new::Student as NewStudent;

generate_create_fn!(students, NewStudent, Student, id, i32);