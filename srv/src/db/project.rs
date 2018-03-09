pub use super::models::Project;
pub use super::models::new::Project as NewProject;

use diesel::result::Error as DieselError;
use super::{DatabaseConnection, SelectError, session};

generate_create_fn!(projects, NewProject, Project);

pub fn get_all_current(conn: &DatabaseConnection) -> Result<Vec<Project>, SelectError> {
    let sess = session::get_latest_session(conn)?;
    let id: i32 = sess.id;
    let projs = generate_select_body!(multi, conn, projects, Project, (session, id))?;
    
    Ok(projs)
}

pub fn get_all(conn: &DatabaseConnection) -> Result<Vec<Project>, SelectError> {
    generate_select_body!(multi, conn, projects, Project)
}

pub fn get_project(conn: &DatabaseConnection, id: i32) -> Result<Project, SelectError> {
    generate_select_body!(single, conn, projects, Project, (id, id))
}

pub fn update_project(conn: &DatabaseConnection, proj: &Project) -> Result<(), DieselError> {
    use diesel::prelude::*;
    proj.save_changes(conn.raw()).map(|_: Project| ())
}