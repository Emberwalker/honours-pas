use rocket::Route;

use config::Config as HPASConfig;

pub fn get_routes(_conf: &HPASConfig) -> Vec<Route> {
    routes![]
}

// TODO: Routes here. Add them to `routes![]` above.
