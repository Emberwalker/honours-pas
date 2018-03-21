v1_imports!();

use rocket::Route;

use db::{project, session, user, staff};

pub fn get_routes() -> Vec<Route> {
    routes![get_sessions_full, new_session]
}

#[get("/sessions/complete")]
fn get_sessions_full(usr: user::User, conn: DatabaseConnection) -> V1Response<SessionFullList> {
    let sessions_fetch = match usr {
        user::User::Staff(_) => session::get_all(&conn),
        user::User::Student(_) => session::get_latest_session(&conn).map(|it| vec![(true, it)]),
    }.map_err(select_error_handler!("no sessions found"))?;
    let sessions = sessions_fetch.into_iter().map(|(current, sess)| SessionEntry {
        session: sess,
        is_current: current,
    }).collect();

    let projects = match usr {
        user::User::Staff(_) => project::get_all(&conn),
        user::User::Student(_) => project::get_all_current(&conn),
    }.map_err(select_error_handler!("no projects found"))?;

    let projects_staffed = project::attach_staff(&conn, projects)
        .map_err(select_error_handler!("error fetching additional staff"))?;

    Ok(Json(SessionFullList {
        sessions: sessions,
        projects: projects_staffed,
    }))
}

#[post("/sessions", data = "<body>")]
fn new_session(
    mut body: Json<session::NewSession>,
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> V1Response<session::Session> {
    body.created = None;
    body.force_archive = None;
    let sess = session::create(&conn, &body).map_err(|e| diesel_error_handler!(e))?;
    Ok(Json(sess))
}