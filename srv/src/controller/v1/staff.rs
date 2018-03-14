v1_imports!();

use std::sync::Arc;

use rocket::{Route, State};

use db::{DatabaseConnection, SelectError};
use db::staff;
use session::SessionManager;

pub fn get_routes() -> Vec<Route> {
    routes![get_staff, rm_staff, new_staff]
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

#[delete("/staff/<id>")]
fn rm_staff(
    id: i32,
    _usr: staff::Admin,
    conn: DatabaseConnection,
    manager: State<Arc<SessionManager>>
) -> Result<Json<GenericMessage>, ErrorResponse> {
    let target = staff::get(&conn, id).map_err(|e| {
        match e {
            SelectError::NoSuchValue() => not_found!("no such staff member"),
            SelectError::DieselError(e) => {
                error!("Diesel error fetching staff member: {}", e);
                debug!("Additional information: {:?}", e);
                internal_server_error!("database error")
            }
        }
    })?;
    staff::delete(&conn, &target).map_err(|e| {
        error!("Diesel error deleting staff member: {}", e);
        debug!("Additional information: {:?}", e);
        internal_server_error!("database error")
    })?;
    manager.remove_session(&target.email);
    Ok(generic_message!("ok"))
}

#[post("/staff", data = "<body>")]
fn new_staff(
    body: Json<NewStaffList>,
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> Result<Json<GenericMessage>, ErrorResponse> {
    staff::create_batch(&conn, &body.staff).map_err(|e| {
        error!("Diesel error creating staff: {}", e);
        debug!("Additional information: {:?}", e);
        internal_server_error!("database error")
    })?;
    Ok(generic_message!("ok"))
}