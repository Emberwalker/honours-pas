v1_imports!();

use std::sync::Arc;

use rocket::{Route, State};

use db::staff;
use session::SessionManager;
use authn::AuthnHolder;

pub fn get_routes() -> Vec<Route> {
    routes![get_staff, rm_staff, new_staff]
}

#[get("/staff")]
fn get_staff(_usr: staff::Admin, conn: DatabaseConnection) -> V1Response<StaffList> {
    match staff::get_all(&conn) {
        Ok(v) => Ok(Json(StaffList { staff: v })),
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
    auth: State<AuthnHolder>,
    manager: State<Arc<SessionManager>>,
) -> V1Response<GenericMessage> {
    let target = staff::get(&conn, id).map_err(select_error_handler!("no such staff member"))?;
    staff::delete(&conn, &target).map_err(|e| diesel_error_handler!(e))?;
    manager.remove_session(&target.email, &auth);
    Ok(generic_message!("ok"))
}

#[post("/staff", data = "<body>")]
fn new_staff(
    body: Json<NewStaffList>,
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> V1Response<GenericMessage> {
    staff::create_batch(&conn, &body.staff).map_err(|e| diesel_error_handler!(e))?;
    Ok(generic_message!("ok"))
}
