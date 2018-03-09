v1_imports!();

use rocket::Route;

use db::{DatabaseConnection, SelectError};
use db::staff;

pub fn get_routes() -> Vec<Route> {
    routes![get_staff, new_staff]
}

#[get("/staff")]
fn get_staff(
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> Result<Json<StaffList>, ErrorResponse> {
    match staff::get_all(&conn) {
        Ok(v) => Ok(Json(StaffList {
            staff: v,
        })),
        Err(e) => {
            error!("Unable to fetch staff: {:?}", e);
            Err(internal_server_error!("database error"))
        }
    }
}

#[post("/staff", data = "<body>")]
fn new_staff(
    body: Json<NewStaffList>,
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> Result<Json<GenericMessage>, ErrorResponse> {
    unimplemented!()
}