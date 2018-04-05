v1_imports!();

use rocket::response::{Responder, Result};
use rocket::{Catcher, Error, Request};

// Note for this we're _manually_ building catchers. Until Rocket 0.4, the error catcher attribute `error(...)`
// conflicts with the `log` crate `error!(...)` macro. Once Rocket 0.4 is out, `error` becomes `catch` and fixes it.
// See: https://github.com/SergioBenitez/Rocket/commit/237c673be449b879234633c32aaa8d70c21c45cd
pub fn get_catchers() -> Vec<Catcher> {
    vec![
        Catcher::new(400, bad_request_400),
        Catcher::new(401, unauthorized_401),
        Catcher::new(403, forbidden_403),
        Catcher::new(404, not_found_404),
        Catcher::new(500, internal_server_error_500),
        Catcher::new(501, not_implemented_501),
    ]
}

fn bad_request_400<'r>(_: Error, req: &'r Request) -> Result<'r> {
    let res = bad_request!("bad request");
    res.respond_to(req)
}

fn unauthorized_401<'r>(_: Error, req: &'r Request) -> Result<'r> {
    let res = unauthorized!("unauthorized");
    res.respond_to(req)
}

fn forbidden_403<'r>(_: Error, req: &'r Request) -> Result<'r> {
    let res = forbidden!("forbidden");
    res.respond_to(req)
}

fn not_found_404<'r>(_: Error, req: &'r Request) -> Result<'r> {
    let res = not_found!("not found");
    res.respond_to(req)
}

fn internal_server_error_500<'r>(_: Error, req: &'r Request) -> Result<'r> {
    let res = internal_server_error!("internal server error");
    res.respond_to(req)
}

fn not_implemented_501<'r>(_: Error, req: &'r Request) -> Result<'r> {
    let res = not_implemented!("not implemented");
    res.respond_to(req)
}
