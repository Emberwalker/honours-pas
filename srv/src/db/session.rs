pub use super::models::Session;
use super::models::new::Session as NewSession;

generate_create_fn!(sessions, NewSession, Session, id, i32);