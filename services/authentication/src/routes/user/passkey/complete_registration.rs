use crate::routes::user::providers::success::login_success;
use crate::ProviderConfiguration;
use actix_web::{post, web, HttpResponse};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use common::entities::prelude::{AuthSession, Passkey, User};
use common::entities::{auth_session, passkey, user};
use common::types::authentication::passkey_complete::PasskeyCompleteRequest;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use webauthn_rs::prelude::{PasskeyRegistration, RegisterPublicKeyCredential};
use webauthn_rs::Webauthn;

#[post("/register/complete")]
pub async fn complete(
    database: web::Data<DatabaseConnection>,
    webauth: web::Data<Webauthn>,
    payload: web::Json<PasskeyCompleteRequest>,
    provider_config: web::Data<ProviderConfiguration>,
) -> HttpResponse {
    let state = match AuthSession::find()
        .filter(auth_session::Column::UserId.eq(payload.user_id.clone()))
        .one(database.as_ref())
        .await
    {
        Ok(state) => state,
        Err(err) => {
            return HttpResponse::BadRequest().body(format!("{:?}", err));
        }
    };

    if state.is_none() {
        return HttpResponse::BadRequest().body("Failed to find user".to_string());
    }

    let state = state.unwrap();
    let registration = serde_json::to_value(state);

    if registration.is_err() {
        return HttpResponse::BadRequest().body("Failed to serialize ccr".to_string());
    }

    let registration = registration.unwrap();
    let registration = &registration["state_data"];

    let passkey_registration: PasskeyRegistration =
        match serde_json::from_value(registration.clone()) {
            Ok(passkey_registration) => passkey_registration,
            Err(err) => {
                return HttpResponse::BadRequest()
                    .body(format!("Failed to deserialize ccr: {}", err));
            }
        };

    let deserialized: RegisterPublicKeyCredential =
        match serde_json::from_value(payload.data.clone()) {
            Ok(deserialized) => deserialized,
            Err(err) => {
                return HttpResponse::BadRequest()
                    .body(format!("Failed to deserialize ccr: {}", err));
            }
        };

    let result = match webauth.finish_passkey_registration(&deserialized, &passkey_registration) {
        Ok(result) => result,
        Err(err) => {
            return HttpResponse::BadRequest().body(format!("{:?}", err));
        }
    };

    match User::insert(user::ActiveModel {
        id: Set(payload.user_id.clone()),
        email: Set(payload.email.clone()),
        github_id: Default::default(),
        google_id: Default::default(),
        username: Set(payload.username.clone()),
        avatar_url: Set(Some(payload.avatar_url.clone())),
        created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
        updated_at: Default::default(),
    })
    .exec(database.as_ref())
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::BadRequest().body(format!("{:?}", err));
        }
    }

    match Passkey::insert(passkey::ActiveModel {
        cred_id: Set(BASE64_STANDARD.encode(result.cred_id())),
        user_id: Set(payload.user_id.clone()),
        passkey_data: Set(serde_json::to_value(&result).unwrap()),
        created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
    })
    .exec(database.as_ref())
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError().json(format!("{:?}", err));
        }
    }

    match AuthSession::delete_many()
        .filter(auth_session::Column::UserId.eq(payload.user_id.clone()))
        .exec(database.as_ref())
        .await
    {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError().json(format!("{:?}", err));
        }
    };

    login_success(
        payload.user_id.clone(),
        provider_config.jwt_secret.clone(),
        provider_config.domain_root.clone(),
        database.get_ref().clone(),
    )
    .await
}
