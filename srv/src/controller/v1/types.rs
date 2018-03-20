use rocket::response::status;
use rocket_contrib::Json;

use db::session::Session;
use db::project::Project;
use db::staff::{Staff, NewStaff};
use db::student::{Student, NewStudent};

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
    pub projects: Vec<Project>,
}

#[derive(Serialize, Debug)]
pub struct ProjectList {
    pub projects: Vec<Project>,
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
    pub students: Vec<NewStudent>,
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
    pub id: i32,
    pub weight: f64,
}

#[derive(Deserialize, Debug)]
pub struct CommentMessage {
    pub comment: Option<String>,
}