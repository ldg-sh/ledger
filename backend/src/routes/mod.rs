use crate::middleware::authentication::Authentication;
use actix_web::web;

mod download;
mod upload;
mod test;
mod file;
mod list;
mod bulk;
mod directory;

static FILE_SCOPE: &str = "/{file_id}";

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/upload")
            .wrap(Authentication)
            .service(web::scope("/create")
                .service(upload::create_upload))
            .service(upload::upload),
    );

    cfg.service(
        web::scope("/list")
            .wrap(Authentication)
            .service(list::list_files),
    );

    cfg.service(
        web::scope("/download")
            .wrap(Authentication)
            .service(
                web::scope(FILE_SCOPE)
                    .service(download::download_full)
                    .service(download::download)
            )
    );

    cfg.service(
        web::scope("/file")
            .wrap(Authentication)
            .service(web::scope(FILE_SCOPE)
                .service(file::delete)
                .service(file::rename)
                .service(file::r#move)
                .service(file::copy)
            ),
    );
    
    cfg.service(
        web::scope("/directory")
            .wrap(Authentication)
            .service(directory::create)
            .service(directory::delete)
            .service(directory::rename)
            .service(directory::copy)
    );

    cfg.service(
        web::scope("/bulk")
            .wrap(Authentication)
            .service(bulk::delete)
            .service(bulk::r#move)
            .service(bulk::copy)
    );

    cfg.service(
        web::scope("/test")
            .service(test::test)
    );
}
