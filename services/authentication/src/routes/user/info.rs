use crate::routes::user::providers::database::ProviderExtension;
use actix_web::{web, HttpResponse};
use common::types::user_info::UserInfoRequest;
use sea_orm::DatabaseConnection;
use std::io::ErrorKind;

#[actix_web::post("info")]
pub async fn info(
    _req: actix_web::HttpRequest,
    payload: web::Json<UserInfoRequest>,
    database: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let user = database.get_user_information(payload.account_id.clone()).await;

    match user {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::InternalServerError().body("Failed to retrieve user information")
            }
        }
    }
}