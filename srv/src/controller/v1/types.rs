use db::models::Project;

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
