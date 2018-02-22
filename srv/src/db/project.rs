pub use super::models::Project;
use super::models::new::Project as NewProject;

generate_create_fn!(projects, NewProject, Project, id, i32);