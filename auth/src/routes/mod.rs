use crate::utils::webutils::{validate_admin_token, validate_token};
use actix_web::web;

pub mod fail;
pub mod health;
pub mod user;
pub mod validate;

// TODO:
// Route auth still needs refinement once we add richer roles/scopes beyond simple token validation.

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let user_auth = actix_web_httpauth::middleware::HttpAuthentication::bearer(validate_token);
    let admin_auth = actix_web_httpauth::middleware::HttpAuthentication::bearer(validate_admin_token);

    // Anything on the /health endpoint
    cfg.service(web::scope("/health").service(health::health));

    // Anything in this .service block
    // is on the /user endpoint
    cfg.service(
        web::scope("/user")
            // user/create
            .service(web::scope("/create").service(user::create::create))
            .wrap(admin_auth)
            // user/regenerate
            .service(
                web::scope("/regenerate")
                    .service(user::regenerate::regenerate)
                    .wrap(user_auth.clone()),
            ),
    );

    // Anything on the /validate endpoint
    cfg.service(web::scope("/validate").service(validate::validate));

    cfg.service(web::scope("/fail").service(fail::fail));
}
