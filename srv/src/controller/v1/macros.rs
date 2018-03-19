/// Used by all v1 modules to get required things in scope for macro expansion.
macro_rules! v1_imports {
    () => (
        use rocket::response::status;
        use rocket::http::Status;
        use rocket_contrib::Json;

        #[allow(unused_imports)]
        use db::{DatabaseConnection, SelectError};
        use controller::v1::types::*;
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

macro_rules! ok {
    ($( $arg:tt )*) => (generic_error!(Status::Ok, $($arg),*))
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

macro_rules! diesel_error_handler {
    ($e:ident) => ({
        error!("Diesel error: {}", $e);
        debug!("Additional information: {:?}", $e);
        internal_server_error!("database error")
    })
}

macro_rules! select_error_handler {
    ($( $arg:tt )*) => (|e| match e {
        SelectError::NoSuchValue() => not_found!($($arg),*),
        SelectError::DieselError(e) => diesel_error_handler!(e),
    })
}