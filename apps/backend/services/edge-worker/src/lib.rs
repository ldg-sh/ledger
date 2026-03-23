use crate::types::configuration::Configuration;
use worker::{event, Context, Env, Request, Response, Router};

pub mod authentication;
pub mod types;
pub mod routes;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response, worker::Error> {
    let config = Configuration::gather_configuration();
    let router = Router::with_data(config);

    router
        .get_async("/metadata", routes::metadata::handle_metadata)
        .post_async("/upload/create", routes::upload::handle_create)
        .post_async("/upload/complete", routes::upload::handle_complete)
        .post_async("/download/create", routes::download::handle_create)
        .get_async("/user/info", routes::user::handle_info)
        .run(req, env)
        .await
}
