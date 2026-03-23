use crate::routes::user::*;
use crate::routes::file::*;
use actix_web::web;

pub mod user;
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
                    .service(list::list)
                    .service(copy::copy)
                    .service(delete::delete)
                    .service(directory::directory)
                    .service(metadata::metadata)
                    .service(r#move::r#move)
                    .service(rename::rename),
            )
            .service(web::scope("/user").service(info::info)),
    );
}
