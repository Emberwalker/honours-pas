pub use super::models::Staff;
use super::models::new::Staff as NewStaff;

generate_create_fn!(staff, NewStaff, Staff, id, i32);