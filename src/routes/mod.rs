use crate::middleware::authentication::validate_token;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

mod delete;
mod download;
mod setup;
mod upload;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validate_token);

    cfg.service(
        web::scope("/upload")
            .service(upload::upload)
            .service(upload::create_upload)
            .wrap(auth.clone()),
    );

    cfg.service(
        web::scope("/download")
            .service(download::metadata)
            .service(download::download)
            .service(download::download_full)
            .service(download::list_all_downloads)
            .wrap(auth.clone()),
    );

    cfg.service(
        web::scope("/delete")
            .service(delete::delete)
            .wrap(auth.clone()),
    );

    cfg.service(
        web::scope("/setup")
            .service(setup::setup)
            .wrap(auth.clone()),
    );
}
