use super::{staff, student, DatabaseConnection, SelectError};

#[derive(Debug)]
pub enum User {
    Student(student::Student),
    Staff(staff::Staff),
}

pub fn find_user(conn: &DatabaseConnection, email: &str) -> Option<User> {
    match staff::find_email(conn, email) {
        Ok(s) => Some(User::Staff(s)),
        Err(SelectError::NoSuchValue()) => match student::find_email(conn, email) {
            Ok(s) => Some(User::Student(s)),
            Err(SelectError::NoSuchValue()) => None,
            Err(SelectError::DieselError(e)) => {
                error!("Diesel error: {}", e);
                None
            }
        },
        Err(SelectError::DieselError(e)) => {
            error!("Diesel error: {}", e);
            None
        }
    }
}
