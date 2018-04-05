use rand::{OsRng, Rng};
use ring_pwhash::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use rocket::Request;
use rocket::http::Status;
use rocket::http::hyper::header;
use rocket::response::content::Html;
use rocket::response::{Responder, Response};
use std::io;

lazy_static! {
    // Based on https://blog.filippo.io/the-scrypt-parameters/ for 2017
    static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(15, 8, 1);
}

// TODO: Use a shared RNG (thread local?) when rand 0.5 arrives.
pub fn generate_rand_string(len: usize) -> String {
    OsRng::new()
        .expect("OS RNG")
        .gen_ascii_chars()
        .take(len)
        .collect::<String>()
}

pub fn check_password(passwd: &str, from_db: &str) -> bool {
    scrypt_check(passwd, from_db).is_ok()
}

pub fn hash_password(passwd: &str) -> io::Result<String> {
    scrypt_simple(passwd, &SCRYPT_PARAMS)
}

pub fn sanitise_email(uname: &str) -> Result<String, ()> {
    let username = uname.to_string();
    let mut matches = username.splitn(2, '@');
    let err_closure = || {
        warn!("Error parsing username as email: '{}'", username);
        ()
    };
    let u1 = matches.next().ok_or_else(&err_closure)?;
    let u2 = matches.next().ok_or_else(&err_closure)?;
    Ok(format!("{}@{}", u1.replace(".", ""), u2))
}

/// Rocket Responder to issue a redirect with HTML body (in case the browser doesn't redirect).
/// This implementation is derived from the default Rocket Redirect responder; see the docs for that for details.
#[derive(Debug)]
pub struct RedirectWithBody(Status, String);

#[allow(dead_code)]
impl RedirectWithBody {
    /// https://api.rocket.rs/rocket/response/struct.Redirect.html#method.to
    pub fn to(url: &str) -> Self {
        RedirectWithBody(Status::SeeOther, String::from(url))
    }

    /// https://api.rocket.rs/rocket/response/struct.Redirect.html#method.temporary
    pub fn temporary(url: &str) -> Self {
        RedirectWithBody(Status::TemporaryRedirect, String::from(url))
    }

    /// https://api.rocket.rs/rocket/response/struct.Redirect.html#method.permanent
    pub fn permanent(url: &str) -> Self {
        RedirectWithBody(Status::PermanentRedirect, String::from(url))
    }

    /// https://api.rocket.rs/rocket/response/struct.Redirect.html#method.found
    pub fn found(url: &str) -> Self {
        RedirectWithBody(Status::Found, String::from(url))
    }

    /// https://api.rocket.rs/rocket/response/struct.Redirect.html#method.moved
    pub fn moved(url: &str) -> Self {
        RedirectWithBody(Status::MovedPermanently, String::from(url))
    }
}

impl Responder<'static> for RedirectWithBody {
    fn respond_to(self, req: &Request) -> Result<Response<'static>, Status> {
        let nested = Html(format!(
            "<head></head><body>Redirecting... If this does not work, click <a href=\"{}\">here.</a></body>",
            self.1
        ));
        Response::build()
            .merge(nested.respond_to(req)?)
            .status(self.0)
            .header(header::Location(self.1))
            .ok()
    }
}

macro_rules! concat_vec {
    [$( $x:expr ),*$(,)*] => ({
        let mut v = Vec::new();
        $(
            v.append(&mut $x);
        )*
        v
    })
}
