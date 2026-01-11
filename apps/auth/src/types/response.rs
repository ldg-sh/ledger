use crate::types::error::AppError;
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

pub enum ApiResponse<T> {
    Ok(T),
    EmptyOk,
    Created(T),
    NoContent,
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse {
        match self {
            ApiResponse::Ok(v) => HttpResponse::Ok().json(v),
            ApiResponse::EmptyOk => HttpResponse::Ok().finish(),
            ApiResponse::Created(v) => HttpResponse::Created().json(v),
            ApiResponse::NoContent => HttpResponse::NoContent().finish(),
        }
    }
}

pub type ApiResult<T> = Result<ApiResponse<T>, AppError>;
