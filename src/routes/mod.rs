use actix_web::web;
use crate::middleware::authentication::Authentication;
use crate::middleware::authorization::Authorization;

mod delete;
mod download;
mod setup;
mod upload;

static FILE_SELECTION_SCOPE: &str = "/{team}/{key}";

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/upload")
            .service(
                web::scope("/{team}/create")
                    .wrap(Authorization)
                    .wrap(Authentication)
                    .service(upload::create_upload)
            )
            .service(
                web::scope(FILE_SELECTION_SCOPE)
                    .wrap(Authorization)
                    .wrap(Authentication)
                    .service(upload::upload)
            ));


    cfg.service(
        web::scope("/download")
            .wrap(Authorization)
            .wrap(Authentication)
            .service(web::scope(FILE_SELECTION_SCOPE)
                .service(download::metadata)
                .service(download::download)
                .service(download::download_full)
                .service(download::list_all_downloads)
    ));

    cfg.service(
        web::scope("/delete")
            .service(web::scope(FILE_SELECTION_SCOPE)
                .wrap(Authorization)
                .wrap(Authentication)
                .service(delete::delete)
            )
    );

    cfg.service(
        web::scope("/setup")
            .wrap(Authentication)
            .service(setup::setup)
    );
}
