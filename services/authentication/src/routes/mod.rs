use actix_web::web;

pub mod metadata;
mod upload;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/internal").service(
            web::scope("/upload")
                .service(upload::init)
                .service(upload::complete),
        )
    );
}
