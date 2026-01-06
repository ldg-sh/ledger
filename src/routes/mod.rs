use crate::middleware::authentication::Authentication;
use actix_web::web;

mod delete;
mod download;
mod upload;
mod test;

static FILE_SCOPE: &str = "/{path:.*}";

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/upload")
            .wrap(Authentication)
            .service(web::scope("/create").service(upload::create_upload))
            .service(upload::upload),
    );

    cfg.service(
        web::scope("/list")
            .wrap(Authentication)
            .service(download::list_files),
    );

    cfg.service(
        web::scope("/download")
            .wrap(Authentication)
            .service(download::metadata)
            .service(download::download)
            .service(download::download_full),
    );

    cfg.service(
        web::scope("/delete")
            .wrap(Authentication)
            .service(web::scope(FILE_SCOPE).service(delete::delete)),
    );

    cfg.service(
        web::scope("/test")
            .service(test::test)
    );
}
