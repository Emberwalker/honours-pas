use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Duration as ChronoDuration;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use rand::os::OsRng;
use rand::Rng;

use config::Config;

#[cfg(feature = "insecure")]
const SECURED: bool = false;

#[cfg(not(feature = "insecure"))]
const SECURED: bool = true;

#[derive(Clone, Debug)]
pub struct Session {
    email: String,
    created: Instant,
}

#[derive(Debug)]
pub struct SessionManager {
    max_age: Duration,
    sessions: RwLock<HashMap<String, Session>>,
}

impl SessionManager {
    pub fn new(conf: &Config) -> Arc<Self> {
        let expiry = conf.get_session_expiry();
        let arc = Arc::new(SessionManager {
            max_age: ChronoDuration::minutes(i64::from(expiry)).to_std().expect("Chrono to std::time"),
            sessions: RwLock::new(HashMap::new()),
        });
        let arc_clone = Arc::clone(&arc);

        // Spawn cleaner thread.
        thread::Builder::new().name("session-cleanup".to_string()).spawn(move || {
            let duration = Duration::from_secs(60);
            loop {
                thread::sleep(duration);
                let mut vec = Vec::<String>::new();
                let now = Instant::now();
                { // Read block
                    let sessions = arc_clone.sessions.read().unwrap();
                    for (k, v) in sessions.iter() {
                        if now.duration_since(v.created) > arc_clone.max_age {
                            vec.push(k.to_string());
                        }
                    }
                } // Read block END
                if vec.len() > 0 {
                    info!("Purging {} expired sessions.", vec.len());
                    let mut sessions = arc_clone.sessions.write().unwrap();
                    for id in vec.drain(..) {
                        sessions.remove(&id);
                    }
                }
            }
        }).expect("cleanup thread creation");

        arc
    }

    /// Creates a new session, adds it to the manager and client cookies. Returns a copy of the new session.
    pub fn new_session(&self, email: &str, cookies: &mut Cookies) -> Session {
        let session = Session {
            email: email.to_string(),
            created: Instant::now(),
        };
        let session_cloned = session.clone();

        let key = SessionManager::generate_new_session_key();
        {
            let mut sessions = self.sessions.write().unwrap();
            sessions.insert(key.clone(), session);
        }
        let cookie = Cookie::build("session", key).secure(SECURED).http_only(true).finish();
        cookies.add_private(cookie);

        session_cloned
    }

    fn generate_new_session_key() -> String {
        OsRng::new().expect("OS RNG").gen_ascii_chars().take(32).collect::<String>()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Session, ()> {
        let manager = request.guard::<State<Arc<SessionManager>>>()?;
        let cookie = request.cookies().get_private("session");
        match cookie {
            None => Outcome::Failure((Status::Forbidden, ())),
            Some(c) => {
                let sessions = manager.sessions.read().unwrap();
                match sessions.get(c.value()) {
                    None => Outcome::Failure((Status::Forbidden, ())),
                    Some(sess) => Outcome::Success(sess.clone())
                }
            }
        }
    }
}