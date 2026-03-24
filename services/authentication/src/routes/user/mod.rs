use actix_web::web;
use crate::routes::user::providers::{github, google};

pub mod providers;
pub mod refresh;
pub mod info;
pub mod logout;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::scope("/callback")
                .service(github::github_callback)
                .service(google::google_callback)
            )
            .service(
                logout::logout
            )
            .service(
                info::info
            )
            .service(
                refresh::refresh
            )
    );
}