use diesel;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::sync::Arc;

use config::Config;

/// Generates basic CRUD functions. Fields can be passed at the end to enable upsert based on that field.
/// Upsert entries look like `(field_name, another_field_name -> other_field, another_field)` - the fields before `->`
/// are the conflicting fields (usually PKs), and the fields after are the ones to update on conflict.
/// Passing `noupdate` as the last parameter disables creating the `update` call - this is useful for tables which can't
/// implement AsChangeset (such as tables with only PKs).
macro_rules! generate_crud_fns {
    ($table:ident, $new_type:ty, $model_type:ty $(, ($($up_field:ident),+ -> $($re_field:ident),+))*, noupdate) => (
        use diesel;
        use db;

        #[allow(dead_code)] // May not be used for all modules
        pub fn create(
            conn: &db::DatabaseConnection,
            val: &$new_type,
        ) -> Result<$model_type, diesel::result::Error> {
            use diesel::prelude::*;
            use diesel::insert_into;
            #[allow(unused_imports)] // This is only used if upserts are being used in this macro call.
            use diesel::pg::upsert::*;
            use schema::$table;

            debug!(target: concat!("macro_gen::db::", stringify!($table)), "INSERT/pre: {:?}", val);

            #[allow(double_parens)] // Required for some evaluations of the macro
            let res = insert_into($table::table)
                .values(val)
                $(
                    .on_conflict((
                        $(
                            $table::$up_field
                        ),+
                    ))
                    .do_update()
                    .set((
                        $(
                            $table::$re_field.eq(excluded($table::$re_field)),
                        )+
                    ))
                )*
                .get_result::<$model_type>(conn.raw())
                .map_err(|err| {
                    debug!(target: concat!("macro_gen::db::", stringify!($table)), "INSERT/err: {:?}", err);
                    err
                })?;
            debug!(target: concat!("macro_gen::db::", stringify!($table)), "INSERT/post: {:?}", res);
            Ok(res)
        }

        #[allow(dead_code)]
        pub fn create_batch(conn: &db::DatabaseConnection, val: &[$new_type]) -> Result<(), diesel::result::Error> {
            use diesel::prelude::*;
            use diesel::insert_into;
            #[allow(unused_imports)] // This is only used if upserts are being used in this macro call.
            use diesel::pg::upsert::*;
            use schema::$table;

            debug!(target: concat!("macro_gen::db::", stringify!($table), "::batch"), "INSERT/pre: {:?}", val);

            #[allow(double_parens)] // Required for some evaluations of the macro
            let res = insert_into($table::table)
                .values(val)
                $(
                    .on_conflict((
                        $(
                            $table::$up_field
                        ),+
                    ))
                    .do_update()
                    .set((
                        $(
                            $table::$re_field.eq(excluded($table::$re_field)),
                        )+
                    ))
                )*
                .execute(conn.raw())
                .map_err(|err| {
                    debug!(target: concat!("macro_gen::db::", stringify!($table), "::batch"), "INSERT/err: {:?}", err);
                    err
                })?;
            debug!(target: concat!("macro_gen::db::", stringify!($table), "::batch"), "INSERT/post: {} records.", res);
            Ok(())
        }

        #[allow(dead_code)]
        pub fn delete(conn: &db::DatabaseConnection, val: &$model_type) -> Result<(), diesel::result::Error> {
            use diesel::prelude::*;
            use diesel::Identifiable;
            use schema::$table;

            diesel::delete($table::table.find(val.id())).execute(conn.raw()).map(|_| ())
        }
    );
    ($table:ident, $new_type:ty, $model_type:ty $(, ($($up_field:ident),+ -> $($re_field:ident),+))*) => (
        generate_crud_fns!($table, $new_type, $model_type $(, ($($up_field),+ -> $($re_field),+))*, noupdate);

        #[allow(dead_code)]
        pub fn update(conn: &db::DatabaseConnection, val: &$model_type) -> Result<$model_type, diesel::result::Error> {
            use diesel::prelude::SaveChangesDsl;
            val.save_changes(conn.raw())
        }
    )
}

/// Generates the body of a SELECT function.
macro_rules! generate_select_body {
    (single, $conn:ident, $table:ident, $model_type:ty $(, ($field:ident, $val:ident))*) => (
        {
            let vals = generate_select_body!(
                multi,
                $conn,
                $table,
                $model_type,
                $(
                    ($field, $val)
                ),*)?;
            match vals.get(0).cloned() {
                None => Err(SelectError::NoSuchValue()),
                Some(e) => Ok(e.clone()),
            }
        }
    );
    (multi, $conn:ident, $table:ident, $model_type:ty$(, ($field:ident, $val:ident))*) => (
        {
            use diesel;
            let vals = generate_select_body!(
                __in,
                $conn,
                $table,
                $model_type,
                $(
                    ($field, $val)
                ),*);
            vals
                .map_err(|e| {
                    match e {
                        diesel::result::Error::NotFound => SelectError::NoSuchValue(),
                        e => SelectError::DieselError(e),
                    }
                })
        }
    );
    (__in, $conn:ident, $table:ident, $model_type:ty, $(($field:ident, $val:ident)),*) => (
        {
            // Note that we do NOT use the DSL. This causes naming conflicts in some cases (like filtering on `id`...)
            use diesel::prelude::*;
            use schema::$table;
            $table::table
            $(
                .filter($table::$field.eq(&$val))
            )*
                .load::<$model_type>($conn.raw())
        }
    );
}

#[derive(Debug)]
pub enum SelectError {
    NoSuchValue(),
    DieselError(diesel::result::Error),
}

impl From<diesel::result::Error> for SelectError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => SelectError::NoSuchValue(),
            e => SelectError::DieselError(e),
        }
    }
}

pub mod models;
pub mod project;
pub mod session;
pub mod staff;
pub mod student;
pub mod user;

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
