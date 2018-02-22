use std::ops::Deref;
use std::sync::Arc;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use config::Config;

macro_rules! generate_create_fn {
    ($table:ident, $new_type:ty, $model_type:ty, $ret_field:ident, $ret_type:ty) => (
        use diesel;
        use db;
        pub fn create(conn: &db::DatabaseConnection, val: &$new_type) -> Result<$ret_type, diesel::result::Error> {
            use diesel::prelude::*;
            use diesel::insert_into;
            use schema::$table;

            debug!(target: concat!("macro_gen::db::", stringify!($table)), "INSERT/pre: {:?}", val);

            let res = insert_into($table::table)
                .values(val)
                .get_result::<$model_type>(conn.raw())
                .map_err(|err| {
                    warn!("Error fetching from '{}': {}", stringify!($table), err);
                    err
                })?;
            debug!(target: concat!("macro_gen::db::", stringify!($table)), "INSERT/post: {:?}", res);
            Ok(res.$ret_field)
        }
    )
}

pub mod models;
pub mod staff;
pub mod student;
pub mod session;
pub mod project;

// The following is based on Rocket's guide on integrating DB connection pooling.
// https://rocket.rs/guide/state/#databases

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct DatabaseConnection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

// Request guard for Rocket to provide connections.
impl<'a, 'r> FromRequest<'a, 'r> for DatabaseConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DatabaseConnection, ()> {
        let pool = request.guard::<State<Arc<Pool>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DatabaseConnection(conn)),
            Err(_) => {
                error!("Unable to get database worker!");
                Outcome::Failure((Status::ServiceUnavailable, ()))
            }
        }
    }
}

// For the convenience of using an &DatabaseConnection as a &PgConnection.
impl Deref for DatabaseConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DatabaseConnection {
    pub fn raw(&self) -> &PgConnection {
        &self.0
    }
}

pub fn init_pool(conf: &Config) -> Pool {
    info!("Starting database connection pool.");
    let manager = ConnectionManager::<PgConnection>::new(conf.get_database_str());
    r2d2::Pool::new(manager).expect("DB pool creation")
}
