use crate::routes::user::providers::{github, google};
use actix_web::web;

pub mod info;
pub mod logout;
pub mod passkey;
pub mod providers;
pub mod refresh;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::scope("/callback")
                    .service(github::github_callback)
                    .service(google::google_callback),
            )
            .service(logout::logout)
            .service(info::info)
            .service(refresh::refresh)
            .service(
                web::scope("/passkey")
                    .service(passkey::init_registration::register)
                    .service(passkey::complete_registration::complete)
                    .service(passkey::init_login::auth_init)
                    .service(passkey::complete_login::auth_complete),
            ),
    );
}
