use actix_web::{post, web, HttpResponse};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use common::entities::auth_session::ActiveModel;
use common::entities::{passkey, user};
use common::entities::prelude::{AuthSession, Passkey, User};
use common::types::authentication::passkey_init::{PasskeyInitRequest, PasskeyInitResponse};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use webauthn_rs::prelude::CredentialID;
use webauthn_rs::Webauthn;

#[post("/register/init")]
pub async fn register(
    database: web::Data<DatabaseConnection>,
    webauth: web::Data<Webauthn>,
    payload: web::Json<PasskeyInitRequest>,
) -> HttpResponse {
    let id = Uuid::new_v4();

    let existing_user = User::find()
        .filter(user::Column::Email.eq(payload.email.clone()))
        .all(database.get_ref())
        .await
        .unwrap_or(vec![]);

    if !existing_user.is_empty() {
        return HttpResponse::Conflict().finish()
    }

    let existing_keys = match &payload.existing_id {
        Some(existing_id) => Passkey::find()
            .filter(passkey::Column::UserId.eq(existing_id))
            .all(database.get_ref())
            .await
            .unwrap_or(vec![]),
        None => vec![],
    };

    let binaries = existing_keys
        .iter()
        .map(|key| key.cred_id.clone())
        .collect::<Vec<_>>();

    let cred_ids = binaries
        .iter()
        .map(|key| CredentialID::from(BASE64_STANDARD.decode(key.clone().as_bytes()).unwrap()))
        .collect::<Vec<_>>();

    let (ccr, state) = match webauth.start_passkey_registration(
        id,
        &payload.email.clone(),
        &payload.username.clone(),
        Some(cred_ids),
    ) {
        Ok((ccr, state)) => (ccr, state),
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json("Failed to register credentials".to_string());
        }
    };

    let serialized = match serde_json::to_value(&state) {
        Ok(serialized) => serialized,
        Err(_) => {
            return HttpResponse::InternalServerError().json("Failed to serialize ccr".to_string());
        }
    };

    match AuthSession::insert(ActiveModel {
        user_id: Set(id.to_string()),
        state_data: Set(serialized),
        expires_at: Set(DateTimeWithTimeZone::from(
            chrono::Utc::now() + chrono::Duration::days(1),
        )),
    })
    .exec(database.as_ref())
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError().json(format!("{:?}", err));
        }
    };

    let serialized = match serde_json::to_value(&ccr) {
        Ok(serialized) => serialized,
        Err(_) => {
            return HttpResponse::InternalServerError().json("Failed to serialize ccr".to_string());
        }
    };

    HttpResponse::Ok().json(PasskeyInitResponse {
        user_id: id.to_string(),
        response: serialized
    })
}
