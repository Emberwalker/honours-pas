pub use super::models::Staff;
pub use super::models::new::Staff as NewStaff;

use super::{DatabaseConnection, SelectError};

generate_create_fn!(staff, NewStaff, Staff, id, i32);

pub fn find_email(conn: &DatabaseConnection, staff_email: &str) -> Result<Staff, SelectError> {
    generate_select_body!(single, conn, staff, Staff, (email, staff_email))
}

pub fn get_all(conn: &DatabaseConnection) -> Result<Vec<Staff>, SelectError> {
    generate_select_body!(multi, conn, staff, Staff)
}
