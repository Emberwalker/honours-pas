use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

pub struct ServerHeader();

impl Fairing for ServerHeader {
    fn info(&self) -> Info {
        Info {
            name: "Server Header",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new(
            "Server",
            "Project Allocation Service (Rust/Rocket)",
        ));
    }
}
