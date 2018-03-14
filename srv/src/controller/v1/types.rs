use rocket::response::status;
use rocket_contrib::Json;

use db::project::Project;
use db::staff::{Staff, NewStaff};
use db::student::{Student, NewStudent};

pub type ErrorResponse = status::Custom<Json<GenericMessage>>;

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
    pub user_type: String,
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