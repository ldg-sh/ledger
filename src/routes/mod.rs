use actix_web::web;

mod upload;

pub fn configure_routes(
    cfg: &mut web::ServiceConfig
) {

    cfg
        .service(
            web::scope("/upload")
                .service(upload::upload),
        );
}