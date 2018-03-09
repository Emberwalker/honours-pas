/// Used by all v1 modules to get required things in scope for macro expansion.
macro_rules! v1_imports {
    () => (
        use rocket::response::status;
        use rocket::http::Status;
        use rocket_contrib::Json;
    )
}

macro_rules! generic_message {
    ($( $arg:tt )*) => (
        Json(GenericMessage {
            message: format!($($arg),*),
        })
    )
}

macro_rules! generic_error {
    ($status:path, $( $arg:tt )*) => (
        status::Custom($status, generic_message!($($arg),*))
    )
}

macro_rules! bad_request {
    ($( $arg:tt )*) => (generic_error!(Status::BadRequest, $($arg),*))
}

macro_rules! unauthorized {
    ($( $arg:tt )*) => (generic_error!(Status::Unauthorized, $($arg),*))
}

macro_rules! forbidden {
    ($( $arg:tt )*) => (generic_error!(Status::Forbidden, $($arg),*))
}

macro_rules! not_found {
    ($( $arg:tt )*) => (generic_error!(Status::NotFound, $($arg),*))
}

macro_rules! internal_server_error {
    ($( $arg:tt )*) => (generic_error!(Status::InternalServerError, $($arg),*))
}

macro_rules! not_implemented {
    ($( $arg:tt )*) => (generic_error!(Status::NotImplemented, $($arg),*))
}
