use std::ops::Deref;

use rocket::{Outcome, Request};
use rocket::request::{self, FromRequest};
use rocket::http::Status;

pub use super::models::Staff;
pub use super::models::new::Staff as NewStaff;

use super::{DatabaseConnection, SelectError};
use session::Session;

// Enable upsert on the email field.
generate_create_fn!(staff, NewStaff, Staff, (email -> full_name, is_admin));

pub fn find_email(conn: &DatabaseConnection, staff_email: &str) -> Result<Staff, SelectError> {
    generate_select_body!(single, conn, staff, Staff, (email, staff_email))
}

pub fn get_all(conn: &DatabaseConnection) -> Result<Vec<Staff>, SelectError> {
    generate_select_body!(multi, conn, staff, Staff)
}

impl<'a,'r> FromRequest<'a,'r> for Staff {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Staff, ()> {
        let sess = request.guard::<Session>()?;
        let conn = request.guard::<DatabaseConnection>()?;

        match find_email(&conn, &sess.email) {
            Ok(s) => Outcome::Success(s),
            Err(SelectError::NoSuchValue()) => Outcome::Failure((Status::Forbidden, ())),
            Err(SelectError::DieselError(e)) => {
                error!("Diesel error fetching Staff record: {}", e);
                debug!("Detailed error: {:?}", e);
                Outcome::Failure((Status::InternalServerError, ()))
            }
        }
    }
}

pub struct Admin(pub Staff);

impl Deref for Admin {
    type Target = Staff;

    fn deref(&self) -> &Staff {
        &self.0
    }
}

impl<'a,'r> FromRequest<'a,'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, ()> {
        let s = request.guard::<Staff>()?;
        if s.is_admin {
            Outcome::Success(Admin(s))
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}