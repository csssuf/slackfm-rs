use std::env;
use std::ops::Deref;

#[cfg(feature = "postgres")]
use diesel::pg::PgConnection;
#[cfg(feature = "sqlite")]
use diesel::sqlite::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub mod models;
pub mod schema;

#[cfg(feature = "postgres")]
type Conn = PgConnection;
#[cfg(feature = "sqlite")]
type Conn = SqliteConnection;

pub(crate) type Pool = r2d2::Pool<ConnectionManager<Conn>>;

/// Initializes a database pool.
pub(crate) fn init_pool() -> Result<Pool, env::VarError> {
    let database_url = env::var("SLACKFM_DATABASE_URL")?;
    let manager = ConnectionManager::<Conn>::new(database_url);
    Ok(r2d2::Pool::new(manager).expect("db pool"))
}

pub(crate) struct DbConn(pub(crate) r2d2::PooledConnection<ConnectionManager<Conn>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = Conn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
