use diesel::{ConnectionError, result::{Error::*, DatabaseErrorKind}};
use rocket::{
    serde::{json::Json},
    http::Status,
};

pub type JsonResult<T> = std::result::Result<Json<T>,Status>;

pub trait ToStatus {
    fn to_status(self) -> Status;
}

pub trait MapStatus<R> {
    fn status(self) -> Result<R,Status>;
}

impl<R,E:ToStatus> MapStatus<R> for Result<R,E> {
    fn status(self) -> Result<R,Status> {
        self.map_err(|e| e.to_status())
    }
}

impl ToStatus for ConnectionError {
    fn to_status(self) -> Status {
        Status::ServiceUnavailable
    }
}

impl ToStatus for diesel::result::Error {
    fn to_status(self) -> Status {
        match self {
            NotFound => Status::NotFound,
            // Нарушение уникальности (повторение PK)
            DatabaseError(DatabaseErrorKind::UniqueViolation,_) => Status::Conflict,
            _ => Status::InternalServerError,
        }
    }
}

impl ToStatus for std::env::VarError {
    fn to_status(self) -> Status {
        Status::InternalServerError
    }
}
