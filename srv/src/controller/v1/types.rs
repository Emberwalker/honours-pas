use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;

use db::models::Project;
use db::models::Staff;

#[derive(Serialize, Debug)]
pub struct GenericMessage {
    pub message: String,
}

pub type ErrorResponse = status::Custom<Json<GenericMessage>>;

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