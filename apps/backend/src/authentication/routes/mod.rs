use actix_web::web;
use crate::authentication::routes::providers::{github, google};

pub mod providers;
mod refresh;
mod info;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::scope("/callback")
                .service(github::github_callback)
                .service(google::google_callback)
            )
            .service(
                info::info
            )
            .service(
                refresh::refresh
            )
    );
}