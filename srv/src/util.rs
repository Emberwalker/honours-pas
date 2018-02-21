use ring_pwhash::scrypt::{ScryptParams, scrypt_check, scrypt_simple};

lazy_static! {
    // Based on https://blog.filippo.io/the-scrypt-parameters/ for 2017
    static ref SCRYPT_PARAMS = ScryptParams::new(32768, 8, 1);
}

pub fn check_password(passwd: &str, from_db: &str) -> bool {
    scrypt_check(passwd, from_db).is_ok()
}

pub fn hash_password(passwd: &str) -> Result<String> {
    scrypt_simple(passwd, SCRYPT_PARAMS)
}