use rocket::{Outcome, Request};
use rocket::request::{self, FromRequest};
use rocket::http::Status;

use session::Session;
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

impl User {
    pub fn email(&self) -> String {
        match *self {
            User::Staff(ref s) => s.email.clone(),
            User::Student(ref s) => s.email.clone(),
        }
    }

    pub fn full_name(&self) -> String {
        match *self {
            User::Staff(ref s) => s.full_name.clone(),
            User::Student(ref s) => s.full_name.clone(),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let s = request.guard::<Session>()?;
        let conn = request.guard::<DatabaseConnection>()?;
        match find_user(&conn, &s.email) {
            Some(u) => Outcome::Success(u),
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
