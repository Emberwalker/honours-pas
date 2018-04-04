v1_imports!();

use std::collections::HashMap;

use rocket::Route;
use bigdecimal::BigDecimal;

use db::{project, session, staff, student, user};

pub fn get_routes() -> Vec<Route> {
    routes![get_sessions_full, new_session, archive_session, rm_session, get_session_report]
}

#[get("/sessions/complete")]
fn get_sessions_full(usr: user::User, conn: DatabaseConnection) -> V1Response<SessionFullList> {
    let sessions_fetch = match usr {
        user::User::Staff(_) => session::get_all(&conn),
        user::User::Student(_) => session::get_latest_session(&conn).map(|it| vec![(true, it)]),
    }.map_err(select_error_handler!("no sessions found"))?;
    let sessions = sessions_fetch
        .into_iter()
        .map(|(current, sess)| SessionEntry {
            session: sess,
            is_current: current,
        })
        .collect();

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

#[post("/sessions/<id>/archive")]
fn archive_session(
    id: i32,
    _usr: staff::Admin,
    conn: DatabaseConnection,
) -> V1Response<GenericMessage> {
    let (_, mut sess) =
        session::get_session(&conn, id).map_err(select_error_handler!("no such session"))?;
    sess.force_archive = true;
    let sess = sess;
    session::update(&conn, &sess).map_err(|e| diesel_error_handler!(e))?;
    Ok(generic_message!("ok"))
}

#[delete("/sessions/<id>")]
fn rm_session(id: i32, _usr: staff::Admin, conn: DatabaseConnection) -> V1Response<GenericMessage> {
    let (active, sess) =
        session::get_session(&conn, id).map_err(select_error_handler!("no such session"))?;
    if active {
        return Err(bad_request!(
            "cannot delete active sessions; archive it first."
        ));
    }
    // TODO: Also purge assosciated student records.
    session::delete(&conn, &sess).map_err(|e| diesel_error_handler!(e))?;
    Ok(generic_message!("ok"))
}

#[get("/sessions/<id>/report")]
fn get_session_report(id: i32, _usr: staff::Admin, conn: DatabaseConnection) -> V1Response<SessionReport> {
    let (_, sess) = session::get_session(&conn, id).map_err(select_error_handler!("no such session"))?;
    let mut projects = project::get_all_by_session(&conn, sess.id).map_err(select_error_handler!("no projects found"))?;
    // Down convert from full Project structs to ProjectStripped structs to save memory and bandwidth.
    let projects = projects.drain(..).map(move |p| p.into()).collect::<Vec<ProjectStripped>>();

    let students = student::get_all_by_session(&conn, sess.id).map_err(select_error_handler!("no students found"))?;
    let sels = student::selection::get_all_for_session(&conn, sess.id)
        .map_err(select_error_handler!("no student selections found"))?;

    // Generate by-student breakdown.
    // First bucket students with their choices.
    let mut student_sel_map: HashMap<i32, Vec<(i32, BigDecimal)>> = HashMap::with_capacity(students.len());
    for sel in sels {
        let mut choices = student_sel_map.entry(sel.student).or_insert_with(Vec::new);
        choices.push((sel.project, sel.weight));
    }
    // Sort the bucket contents (descending order)
    for (_k, v) in student_sel_map.iter_mut() {
        v.sort_unstable_by(|ref a, ref b| b.1.cmp(&a.1));
    }
    // Generate by-student entry.
    let by_student = student_sel_map.drain().map(move |(k, vs)| SessionReportByStudent {
        student: k,
        choices: vs.iter().map(|ref it| it.0).collect::<Vec<i32>>(),
        is_eq: vs.windows(2).map(|win| win[0].1 == win[1].1).collect::<Vec<bool>>(),
    }).collect::<Vec<SessionReportByStudent>>();

    // Generate by-project breakdown.
    let mut project_sel_map: HashMap<i32, Vec<Vec<(i32, bool)>>> = HashMap::with_capacity(projects.len());
    // Define helper to fill the vec out to the required length
    fn build_up_to<'a, T>(v: &'a mut Vec<Vec<T>>, depth: usize) -> &'a mut Vec<T>
        where T: Clone
    {
        if v.len() < depth + 1 {
            v.resize(depth + 1, Vec::new());
        }
        v.get_mut(depth).expect("build_up_to")
    }
    // Iterate over by-students data to generate by-projects.
    for student in by_student.iter() {
        if student.choices.len() == 0 { continue; }
        let eq_count = student.is_eq.len();
        let mut prev = 0;
        for (i, proj) in student.choices[..].iter().enumerate() {
            // This nasty tangle resolves equal ranked choices. It does this by doing the equivalent of:
            //   `idx = student.is_eq.iter().take(i).rposition(|it| !*it).unwrap_or(i);`, which doesn't work due to
            // `DoubleEndedIterator` bounds. It then ensures there's no gaps in choices (2 first choices, and 1 _second_
            // choice rather than 2 firsts and a third choice).
            debug!("i: {}, proj {:?}, student {:?}", i, proj, student);
            let mut idx = i - student.is_eq.iter().rev().skip(eq_count - i).position(|it| !*it).unwrap_or(i);

            // Catch if the previous was equal as well as if this one is.
            let mut was_eq = student.is_eq.get(i).map(|it| *it).unwrap_or(false);
            was_eq = was_eq || student.is_eq.get(i-1).map(|it| *it).unwrap_or(false);

            if idx - prev > 1 { idx = prev + 1; }
            prev = idx;
            build_up_to(project_sel_map.entry(*proj).or_insert_with(Vec::new), idx).push((student.student, was_eq));
        }
    }
    let by_project = project_sel_map.drain()
        //.inspect(|&(ref k, ref vs)| debug!("P: {:?} -> {:?}", k, vs))
        .map(move |(k, vs)| SessionReportByProject {
            project: k,
            selections: vs.iter().map(|it| it.iter().map(|ref it| it.0).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>(),
            is_eq: vs.iter().map(|it| it.iter().map(|ref it| it.1).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>(),
        })
        .collect::<Vec<SessionReportByProject>>();

    // Fetch comments
    let mut comments_raw = student::comment::get_all_for_session(&conn, sess.id)
        .map_err(select_error_handler!("unable to find comments"))?;

    let mut comments: HashMap<i32, String> = HashMap::new();
    for c in comments_raw.drain(..) {
        match c.comment {
            Some(comm) => comments.insert(c.student, comm),
            None => continue,
        };
    }

    Ok(Json(SessionReport {
        session: sess,
        by_student: by_student,
        by_project: by_project,
        students: students,
        projects: projects,
        comments: comments,
    }))
}
