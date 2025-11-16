use crate::middleware::authentication::Authentication;
use actix_web::web;

mod delete;
mod download;
mod upload;
mod test;

static FILE_SCOPE: &str = "/{file_id}";

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/upload")
            .wrap(Authentication)
            .service(web::scope("/create").service(upload::create_upload))
            .service(web::scope(FILE_SCOPE).service(upload::upload)),
    );

    cfg.service(
        web::scope("/download")
            .wrap(Authentication)
            .service(
                web::scope(FILE_SCOPE)
                    .service(download::metadata)
                    .service(download::download)
                    .service(download::download_full),
            )
            .service(download::list_all_downloads),
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
