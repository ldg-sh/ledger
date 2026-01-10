use crate::middleware::authentication::Authentication;
use actix_web::web;

mod bulk;
mod create;
mod download;
mod file;
mod list;
mod test;
mod upload;

static FILE_SCOPE: &str = "/{file_id}";

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
            .service(list::list_files),
    );

    cfg.service(
        web::scope("/download").wrap(Authentication).service(
            web::scope(FILE_SCOPE)
                .service(download::download_full)
                .service(download::metadata)
                .service(download::download),
        ),
    );

    cfg.service(
        web::scope("/file").wrap(Authentication).service(
            web::scope(FILE_SCOPE)
                .service(file::delete_file)
                .service(file::rename_file)
                .service(file::r#move),
        ),
    );

    cfg.service(
        web::scope("/create")
            .wrap(Authentication)
            .service(create::create_directory),
    );

    cfg.service(
        web::scope("/bulk").wrap(Authentication).service(
            web::scope(FILE_SCOPE)
                .service(bulk::delete)
                .service(bulk::r#move),
        ),
    );

    cfg.service(web::scope("/test").service(test::test));
}
