use crate::ProviderConfiguration;
use crate::routes::user::providers::success::login_success;
use actix_web::{HttpResponse, post, web};
use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use common::entities::prelude::{AuthSession, Passkey};
use common::entities::{auth_session, passkey};
use common::types::authentication::passkey_auth_complete::PasskeyAuthCompleteRequest;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;
use webauthn_rs::prelude::*;

#[post("/auth/complete")]
pub async fn auth_complete(
    database: web::Data<DatabaseConnection>,
    webauth: web::Data<Webauthn>,
    payload: web::Json<PasskeyAuthCompleteRequest>,
    provider_config: web::Data<ProviderConfiguration>,
) -> HttpResponse {
    let state_row = match AuthSession::find()
        .filter(auth_session::Column::UserId.eq(payload.ticket.clone()))
        .one(database.as_ref())
        .await
    {
        Ok(Some(s)) => s,
        Ok(None) => return HttpResponse::BadRequest().finish(),
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Database error: {:?}", err));
        }
    };

    let auth_state: DiscoverableAuthentication = match serde_json::from_value(state_row.state_data)
    {
        Ok(state) => state,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("State error: {}", err));
        }
    };

    let client_credentials: PublicKeyCredential = match serde_json::from_value(payload.data.clone())
    {
        Ok(cred) => cred,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let ident = match webauth.identify_discoverable_authentication(&client_credentials) {
        Ok(i) => i,
        Err(err) => {
            return HttpResponse::Unauthorized()
                .body(format!("Failed to identify credentials: {}", err));
        }
    };

    let cred_id_search = BASE64_STANDARD.encode(ident.1);

    let stored_row = match Passkey::find()
        .filter(passkey::Column::CredId.eq(cred_id_search.clone()))
        .one(database.as_ref())
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            println!("Failed to find credential with id: {}", cred_id_search);
            return HttpResponse::NotFound().finish();
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Database error: {:?}", err));
        }
    };

    let passkey_definition: webauthn_rs::prelude::Passkey =
        match serde_json::from_value(stored_row.passkey_data) {
            Ok(p) => p,
            Err(err) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to parse stored passkey: {}", err));
            }
        };

    match webauth.finish_discoverable_authentication(
        &client_credentials,
        auth_state,
        &[DiscoverableKey::from(passkey_definition)],
    ) {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::Unauthorized().body(format!("Authentication failed: {}", err));
        }
    };

    let user_handle = match client_credentials.response.user_handle {
        Some(h) => h,
        None => {
            return HttpResponse::Unauthorized()
                .body("Authentication failed: No user handle in response");
        }
    };

    let user_uuid = match Uuid::from_slice(&**user_handle) {
        Ok(u) => u.to_string(),
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to parse user handle as UUID: {}", err));
        }
    };

    match AuthSession::delete_many()
        .filter(auth_session::Column::UserId.eq(user_uuid.clone()))
        .exec(database.as_ref())
        .await
    {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError().json(format!("{:?}", err));
        }
    };

    login_success(
        user_uuid,
        provider_config.jwt_secret.clone(),
        provider_config.domain_root.clone(),
        database.get_ref().clone(),
    )
    .await
}
