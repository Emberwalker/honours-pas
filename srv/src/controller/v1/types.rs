use std::collections::HashMap;

use rocket::response::status;
use rocket_contrib::Json;

use db::session::Session;
use db::project::{Project, ProjectWithStaff};
use db::staff::{NewStaff, Staff};
use db::student::Student;

pub type ErrorResponse = status::Custom<Json<GenericMessage>>;
pub type V1Response<T> = Result<Json<T>, ErrorResponse>;

#[derive(Serialize, Debug)]
pub struct GenericMessage {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginMessage {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct WhoAmIMessage {
    pub email: String,
    pub name: String,
    pub user_type: String,
}

#[derive(Serialize, Debug)]
pub struct SessionEntry {
    pub session: Session,
    pub is_current: bool,
}

#[derive(Serialize, Debug)]
pub struct SessionFullList {
    pub sessions: Vec<SessionEntry>,
    pub projects: Vec<ProjectWithStaff>,
}

#[derive(Serialize, Debug)]
pub struct ProjectList {
    pub projects: Vec<ProjectWithStaff>,
}

#[derive(Serialize, Debug)]
pub struct StaffList {
    pub staff: Vec<Staff>,
}

#[derive(Deserialize, Debug)]
pub struct NewStaffList {
    pub staff: Vec<NewStaff>,
}

#[derive(Serialize, Debug)]
pub struct StudentList {
    pub students: Vec<Student>,
}

#[derive(Deserialize, Debug)]
pub struct NewStudentList {
    pub students: Vec<NewStudentEntry>,
}

#[derive(Deserialize, Debug)]
pub struct NewStudentEntry {
    pub email: String,
    pub full_name: String,
}

#[derive(Deserialize, Debug)]
pub struct MarkMessage {
    pub id: i32,
}

#[derive(Serialize, Debug)]
pub struct MarkList {
    pub projects: Vec<i32>,
}

#[derive(Deserialize, Debug)]
pub struct SelectionList {
    pub selections: Vec<SelectionEntry>,
}

#[derive(Deserialize, Debug)]
pub struct SelectionEntry {
    pub project: i32,
    pub weight: f64,
}

#[derive(Deserialize, Debug)]
pub struct CommentMessage {
    pub comment: Option<String>,
}

/// Version of the Project structure with excess material trimmed to save memory and bandwidth when generating reports.
#[derive(Serialize, Debug)]
pub struct ProjectStripped {
    pub id: i32,
    pub name: String,
    pub supervisor_name: String,
    pub supervisor_email: String,
}

impl From<Project> for ProjectStripped {
    fn from(proj: Project) -> Self {
        ProjectStripped {
            id: proj.id,
            name: proj.name,
            supervisor_name: proj.supervisor_name,
            supervisor_email: proj.supervisor_email,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct SessionReport {
    pub session: Session,
    pub by_student: Vec<SessionReportByStudent>,
    pub by_project: Vec<SessionReportByProject>,
    pub students: Vec<Student>,
    pub projects: Vec<ProjectStripped>,
    pub comments: HashMap<i32, String>,
}

#[derive(Serialize, Debug)]
pub struct SessionReportByStudent {
    pub student: i32,
    /// Array of project IDs, ordered from best to worst.
    pub choices: Vec<i32>,
    /// Array of bools between each pair of choices - true if equal priority, false otherwise.
    /// e.g. `choices = [1, 2, 3]`, `is_eq = [false, true]` for `1 > 2 == 3`
    pub is_eq: Vec<bool>,
}

#[derive(Serialize, Debug)]
pub struct SessionReportByProject {
    pub project: i32,
    /// Map of choice (e.g. 1, 2, 3) -> students with that priority.
    pub selections: Vec<Vec<i32>>,
    /// Map of choice -> array specifying if the matching student marked this selection equal to another.
    pub is_eq: Vec<Vec<bool>>,
}
