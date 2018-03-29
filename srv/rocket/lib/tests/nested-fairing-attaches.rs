#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::State;
use rocket::fairing::AdHoc;
use rocket::http::Method;

#[derive(Default)]
struct Counter {
    attach: AtomicUsize,
    get: AtomicUsize,
}

#[get("/")]
fn index(counter: State<Counter>) -> String {
    let attaches = counter.attach.load(Ordering::Relaxed);
    let gets = counter.get.load(Ordering::Acquire);
    format!("{}, {}", attaches, gets)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(AdHoc::on_attach(|rocket| {
            let counter = Counter::default();
            counter.attach.fetch_add(1, Ordering::Relaxed);
            let rocket = rocket.manage(counter)
                .attach(AdHoc::on_request(|req, _| {
                    if req.method() == Method::Get {
                        let counter = req.guard::<State<Counter>>().unwrap();
                        counter.get.fetch_add(1, Ordering::Release);
                    }
                }));

            Ok(rocket)
        }))
}

mod nested_fairing_attaches_tests {
    use super::*;
    use rocket::local::Client;

    #[test]
    fn test_counts() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("1, 1".into()));

        let mut response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("1, 2".into()));

        client.get("/").dispatch();
        client.get("/").dispatch();
        let mut response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("1, 5".into()));
    }
}
