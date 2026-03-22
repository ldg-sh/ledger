use crate::routes::authentication::info;
use actix_web::web;
use file::list;

pub mod authentication;
pub mod metadata;
pub mod upload;
pub mod file;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/internal")
            .service(
                web::scope("/upload")
                    .service(upload::init)
                    .service(upload::complete),
            )
            .service(
                web::scope("/file")
                    .service(list::list_files)
            )
            .service(web::scope("/user").service(info::info)),
    );
}
