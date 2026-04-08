use crate::routes::file::*;
use crate::routes::user::*;
use actix_web::web;

pub mod file;
pub mod user;

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
                    .service(explode::explode)
                    .service(list::list)
                    .service(copy::copy)
                    .service(delete::delete)
                    .service(web::scope("/directory")
                        .service(directory::directory)
                        .service(delete_directory::delete)
                    )
                    .service(metadata::metadata)
                    .service(r#move::r#move)
                    .service(rename::rename),
            )
            .service(web::scope("/user").service(info::info)),
    );
}
