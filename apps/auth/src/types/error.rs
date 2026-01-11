use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // standard web stuffs
    #[error("already exists")]
    AlreadyExists,
    #[error("not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,

    // infra things
    #[error(transparent)]
    Db(sea_orm::DbErr),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        AppError::from_db(e)
    }
}

#[derive(Serialize)]
struct ErrorBody<'a, 'b> {
    error: &'a str,
    message: &'b str,
}

impl AppError {
    fn kind(&self) -> &'static str {
        match self {
            Self::AlreadyExists => "ALREADY_EXISTS",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict(_) => "CONFLICT",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::Db(_) => "DB_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }
    fn from_db(err: DbErr) -> Self {
        match &err {
            DbErr::RecordNotFound(_) => AppError::NotFound,
            _ => AppError::Db(err),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::AlreadyExists | Self::Conflict(_) => StatusCode::CONFLICT,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Validation(_) | Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Db(_) | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorBody {
            error: self.kind(),
            message: self.kind(),
        })
    }
}
