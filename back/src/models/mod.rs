mod user;
pub use user::*;

use std::env;

use crate::errors::RequestError;

use diesel::prelude::*;
use diesel_logger::LoggingConnection;

pub type Conn = LoggingConnection<SqliteConnection>;
/// Выдаёт Connection для с БД
pub fn get_connection() -> Result<Conn,RequestError> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(LoggingConnection::new(SqliteConnection::establish(&database_url)?))
}
