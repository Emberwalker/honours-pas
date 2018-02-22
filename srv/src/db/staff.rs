pub use super::models::Staff;
use super::models::new::Staff as NewStaff;
use schema::staff;
use db::DatabaseConnection;
use diesel::prelude::*;
use diesel::insert_into;
use diesel::result::Error as DieselErr;

#[allow(dead_code)] // FIXME: Remove this attr once used.
pub fn create_staff(
    conn: &DatabaseConnection,
    email: &str,
    full_name: &str,
    is_admin: bool,
) -> Result<i32, DieselErr> {
    let val = NewStaff {
        email,
        full_name,
        is_admin,
    };
    let res = insert_into(staff::table)
        .values(&val)
        .get_result::<Staff>(conn.raw())?;
    Ok(res.id)
}
