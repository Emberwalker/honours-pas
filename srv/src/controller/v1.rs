use rocket::{Request, Route, Data};
use rocket::handler::Outcome;
use rocket::http::Method::*;

use config::Config as HPASConfig;

pub fn get_routes(_conf: &HPASConfig) -> Vec<Route> {
    routes![]
}

// TODO: Routes here. Add them to `routes![]` above.
