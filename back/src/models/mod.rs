mod user;
use rocket::http::Status;
pub use user::*;

use std::env;


use diesel::prelude::*;
use diesel_logger::LoggingConnection;

use crate::errors::MapStatus;

pub type Conn = LoggingConnection<SqliteConnection>;
/// Выдаёт Connection для с БД
pub fn get_connection() -> Result<Conn,Status> {
    let database_url = env::var("DATABASE_URL").status()?;
    Ok(LoggingConnection::new(SqliteConnection::establish(&database_url).status()?))
}
