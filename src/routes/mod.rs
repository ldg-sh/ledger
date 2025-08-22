use actix_web::web;

mod download;
mod upload;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/upload").service(upload::upload).service(upload::create_upload));
    cfg.service(web::scope("/download")
        .service(download::metadata)
        .service(download::download)
        .service(download::download_full)
        .service(download::list_all_downloads)
    );
}
