pub use super::models::Project;
pub use super::models::new::Project as NewProject;

use super::{DatabaseConnection, SelectError, session};

generate_create_fn!(projects, NewProject, Project, id, i32);

pub fn get_all_current(conn: &DatabaseConnection) -> Result<Vec<Project>, SelectError> {
    let sess = session::get_latest_session(conn)?;
    let id: i32 = sess.id;
    debug!("Get all current: {}", id);
    let projs = generate_select_body!(multi, conn, projects, Project, (session, id))?;
    
    Ok(projs)
}

pub fn get_all(conn: &DatabaseConnection) -> Result<Vec<Project>, SelectError> {
    debug!("Get all");
    generate_select_body!(multi, conn, projects, Project)
}