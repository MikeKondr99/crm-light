use diesel::{ConnectionError, result::{Error::*, DatabaseErrorKind}};
use thiserror::Error;
use rocket::{
    serde::{json::Json},
    http::Status,
    Request,response::{Responder, self}
};

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Не получилось подключится к БД")]
    ConnectionError {
        #[from] source: ConnectionError
    },
    #[error("Ошибка базы данных")]
    DieselError {
        #[from] source: diesel::result::Error
    },
    #[error("Ошибка переменной среды")]
    VarError {
        #[from] source: std::env::VarError
    }
}

pub type JsonResult<T> = std::result::Result<Json<T>,RequestError>;

impl<'r, 'o: 'r> Responder<'r, 'o> for RequestError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        match self {
            RequestError::ConnectionError {..} => Status::ServiceUnavailable,
            RequestError::DieselError { source } => match source {
                // Не найдено в БД
                NotFound => Status::NotFound,
                // Нарушение уникальности (повторение PK)
                DatabaseError(DatabaseErrorKind::UniqueViolation,_) => Status::Conflict,
                _ => Status::InternalServerError,
            }
            RequestError::VarError {..} => Status::InternalServerError
        }.respond_to(request)
    }
}

