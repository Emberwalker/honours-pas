#[derive(Serialize, Debug)]
pub struct GenericMessage {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginMessage {
    pub username: String,
    pub password: String,
}