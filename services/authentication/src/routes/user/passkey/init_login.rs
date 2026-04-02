use actix_web::{post, web, HttpResponse};
use common::entities::auth_session::ActiveModel;
use common::entities::prelude::AuthSession;
use common::types::authentication::passkey_auth_init::PasskeyAuthInitResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use webauthn_rs::Webauthn;

#[post("/auth/init")]
pub async fn auth_init(
    database: web::Data<DatabaseConnection>,
    webauth: web::Data<Webauthn>,
) -> HttpResponse {
    let (ccr, state) = match webauth.start_discoverable_authentication() {
        Ok((ccr, state)) => (ccr, state),
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json("Failed to register credentials".to_string());
        }
    };

    let ticket = Uuid::new_v4();
    let serialized = match serde_json::to_value(&state) {
        Ok(serialized) => serialized,
        Err(_) => {
            return HttpResponse::InternalServerError().json("Failed to serialize ccr".to_string());
        }
    };

    match AuthSession::insert(ActiveModel {
        user_id: Set(ticket.to_string()),
        state_data: Set(serialized),
        state_type: Set("State".to_string()),
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

    HttpResponse::Ok().json(PasskeyAuthInitResponse {
        ccr: serde_json::to_value(ccr).unwrap(),
        state: serde_json::to_value(&state).unwrap(),
        ticket: ticket.to_string(),
    })
}
