pub use super::models::Project;
pub use super::models::ProjectWithStaff;
pub use super::models::new::Project as NewProject;
pub use super::models::new::ProjectWithStaff as NewProjectWithStaff;

use super::models::ProjectStaff;
use super::models::new::ProjectStaff as NewProjectStaff;

use super::{session, DatabaseConnection, SelectError};

generate_crud_fns!(projects, NewProject, Project);

pub fn attach_staff(
    conn: &DatabaseConnection,
    projs: Vec<Project>,
) -> Result<Vec<ProjectWithStaff>, SelectError> {
    use diesel::prelude::*;
    let staff_ents = ProjectStaff::belonging_to(&projs)
        .load::<ProjectStaff>(conn.raw())
        .map_err(|e| match e {
            diesel::result::Error::NotFound => SelectError::NoSuchValue(),
            e => SelectError::DieselError(e),
        })?
        .grouped_by(&projs);
    Ok(projs
        .into_iter()
        .zip(staff_ents)
        .map(move |(p, s)| ProjectWithStaff::from_project(p, s))
        .collect())
}

pub fn create_with_staff(
    conn: &DatabaseConnection,
    ps: &NewProjectWithStaff,
) -> Result<ProjectWithStaff, diesel::result::Error> {
    use diesel::prelude::*;
    use diesel::insert_into;
    use schema::{project_staff, projects};

    let sess = session::get_latest_session(&conn).map_err(|e| match e {
        SelectError::DieselError(e) => e,
        SelectError::NoSuchValue() => diesel::result::Error::NotFound,
    })?;

    // Insert projects - this works like the macro, but we need the ID back!
    let res = insert_into(projects::table)
        .values(&NewProject::from_with_staff(ps.clone(), sess.id))
        .get_result::<Project>(conn.raw())?;

    // Merge the new project ID with its staff members, and insert all of them into project_staff.
    let staff = ps.additional_staff
        .iter()
        .map(|s| NewProjectStaff {
            project: res.id,
            staff: s.clone(),
        })
        .collect();

    let staff_res = insert_into(project_staff::table)
        .values::<&Vec<NewProjectStaff>>(&staff)
        .get_results::<ProjectStaff>(conn.raw())?;

    Ok(ProjectWithStaff::from_project(res, staff_res))
}

//#[allow(dead_code)]
pub fn _create_with_staff_batch(
    conn: &DatabaseConnection,
    ps: Vec<NewProjectWithStaff>,
) -> Result<(), diesel::result::Error> {
    use diesel::prelude::*;
    use diesel::insert_into;
    use schema::{project_staff, projects};

    let sess = session::get_latest_session(&conn).map_err(|e| match e {
        SelectError::DieselError(e) => e,
        SelectError::NoSuchValue() => diesel::result::Error::NotFound,
    })?;

    // Insert projects - this works like the macro, but we need the IDs back!
    let projs: Vec<NewProject> = ps.iter()
        .map(|it| NewProject::from_with_staff(it.clone(), sess.id))
        .collect();
    let res = insert_into(projects::table)
        .values::<&Vec<NewProject>>(&projs)
        .returning(projects::id)
        .get_results(conn.raw())?;

    // Merge the new project IDs with their staff members, and insert all of them into project_staff.
    let staff = res.into_iter()
        .zip(ps)
        .flat_map(|(id, p)| {
            p.additional_staff
                .into_iter()
                .map(move |s| NewProjectStaff {
                    project: id,
                    staff: s,
                })
        })
        .collect();

    insert_into(project_staff::table)
        .values::<&Vec<NewProjectStaff>>(&staff)
        .execute(conn.raw())?;

    Ok(())
}

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
