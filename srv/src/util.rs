use ring_pwhash::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use std::io;

lazy_static! {
    // Based on https://blog.filippo.io/the-scrypt-parameters/ for 2017
    static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(15, 8, 1);
}

pub fn check_password(passwd: &str, from_db: &str) -> bool {
    scrypt_check(passwd, from_db).is_ok()
}

pub fn hash_password(passwd: &str) -> io::Result<String> {
    scrypt_simple(passwd, &SCRYPT_PARAMS)
}

pub fn sanitise_email(uname: &str) -> Result<String, ()> {
    let username = uname.to_string();
    let mut matches = username.splitn(2, "@");
    let err_closure = || {
        warn!("Error parsing username as email: '{}'", username);
        ()
    };
    let u1 = matches.next().ok_or_else(&err_closure)?;
    let u2 = matches.next().ok_or_else(&err_closure)?;
    Ok(format!("{}@{}", u1.replace(".", ""), u2))
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