use crate::ProviderConfiguration;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use common::entities::refresh_token;
use common::util::authentication::generate_access_token;
use sea_orm::sea_query::prelude::chrono;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, ConnectionTrait, DbBackend, Statement};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;
use sea_orm::sea_query::PostgresQueryBuilder;

#[actix_web::post("refresh")]
pub async fn refresh(
    req: actix_web::HttpRequest,
    provider_configuration: web::Data<ProviderConfiguration>,
    database: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let start_time = Utc::now();
    let old_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_string(),
        None => return HttpResponse::Unauthorized().body("No refresh token found"),
    };

    println!("Refresh request received at {}ms after start.", (Utc::now() - start_time).num_milliseconds());

    let new_token = Uuid::new_v4().to_string();
    let new_expiry = Utc::now() + Duration::days(30);

    let (sql, values) = sea_query::Query::update()
        .table(refresh_token::Entity)
        .values([
            (refresh_token::Column::Token, new_token.clone().into()),
            (refresh_token::Column::ExpiresAt, new_expiry.into()),
        ])
        .and_where(refresh_token::Column::Token.eq(old_token.trim()))
        .and_where(refresh_token::Column::ExpiresAt.gt(Utc::now()))
        .returning_col(refresh_token::Column::UserId)
        .build(PostgresQueryBuilder);

    println!("Built the SQL command {}ms after start", (Utc::now() - start_time).num_milliseconds());

    let stmt = Statement::from_sql_and_values(DbBackend::Postgres, sql, values);

    let res = database
        .query_one_raw(stmt)
        .await;

    println!("Executed the SQL command {}ms after start", (Utc::now() - start_time).num_milliseconds());

    if res.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let user_id = match res.unwrap() {
        Some(row) => row.try_get::<String>("", "user_id"),
        None => return HttpResponse::Unauthorized().body("Invalid or expired session"),
    };

    if user_id.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    println!("Refresh successful after {}ms", (Utc::now() - start_time).num_milliseconds());

    let db_clone = database.get_ref().clone();
    tokio::spawn(async move {
        let _ = refresh_token::Entity::delete_many()
            .filter(refresh_token::Column::ExpiresAt.lt(Utc::now()))
            .exec(&db_clone)
            .await;
    });

    println!("Finished queuing deletion {}ms", (Utc::now() - start_time).num_milliseconds());

    let user_id = user_id.unwrap();

    let access_cookie = actix_web::cookie::Cookie::build("session", generate_access_token(&user_id.clone(), &provider_configuration.jwt_secret))
        .path("/")
        .max_age(actix_web::cookie::time::Duration::minutes(15))
        .http_only(true)
        .domain(&provider_configuration.domain_root)
        .secure(true)
        .same_site(actix_web::cookie::SameSite::None)
        .finish();

    let refresh_cookie = actix_web::cookie::Cookie::build("refresh_token", new_token)
        .path("/")
        .secure(true)
        .domain(&provider_configuration.domain_root)
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::days(30))
        .same_site(actix_web::cookie::SameSite::None)
        .finish();

    println!("Built cookies {}ms after start", (Utc::now() - start_time).num_milliseconds());

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(json!({
            "user_id": user_id,
        }))
}