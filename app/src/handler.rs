use axum::{http::StatusCode, response::IntoResponse, Extension, Form};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{debug, info};

use crate::ExtState;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WebhookData {
    pub message_status: String,
    pub sms_sid: String,
    pub to: String,
    pub from: String,
    pub account_sid: String,
}

// #[derive(Deserialize, Debug)]
// pub struct WebhookPayload {
//     pub SmsMessageSid: String,
//     pub NumMedia: String,
//     pub ProfileName: String,
//     pub MessageType: String,
//     pub SmsSid: String,
//     pub WaId: String,
//     pub SmsStatus: String,
//     pub Body: String,
//     pub ButtonText: String,
//     pub To: String,
//     pub ButtonPayload: String,
//     pub NumSegments: String,
//     pub ReferralNumMedia: String,
//     pub MessageSid: String,
//     pub AccountSid: String,
//     pub From: String,
//     pub ApiVersion: String,
// }

// pub async fn handle_twilio_webhook_payload(
//     Extension(app_state): ExtState,
//     Form(form): Form<WebhookPayload>,
// ) {
//     println!("{:?}", form);
// }

pub async fn handle_twilio_webhook_status(
    Extension(app_state): ExtState, // Correct the type for Extension
    Form(form): Form<WebhookData>,
) -> Result<impl IntoResponse, StatusCode> {
    let db: &PgPool = &app_state.db;

    info!(
        "Received webhook data: SmsSid: {:?}, MessageStatus: {:?}",
        form.sms_sid, form.message_status
    );

    tracing::info!("resp: {:?}", form);

    let query_main = r#"
        INSERT INTO twilio_webhook_logs (
            sms_sid, from_phone, to_phone, account_sid, latest_status
        ) VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (sms_sid)
        DO UPDATE SET
            latest_status = EXCLUDED.latest_status,
            updated_at = CURRENT_TIMESTAMP
    "#;

    info!(
        "Inserting/Updating twilio_webhook_logs for SmsSid: {:?}",
        form.sms_sid
    );

    sqlx::query(query_main)
        .bind(form.sms_sid.clone())
        .bind(form.from.clone())
        .bind(form.to.clone())
        .bind(form.account_sid.clone())
        .bind(form.message_status.clone())
        .execute(db)
        .await
        .map_err(|err| {
            tracing::error!("Error creating the webhook log: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let query_status = r#"
        INSERT INTO twilio_message_status (sms_sid, message_status)
        VALUES ($1, $2)
    "#;

    info!(
        "Inserting message status for SmsSid: {:?}, MessageStatus: {:?}",
        form.sms_sid, form.message_status
    );

    sqlx::query(query_status)
        .bind(form.sms_sid.clone())
        .bind(form.message_status)
        .execute(db)
        .await
        .map_err(|err| {
            tracing::error!("Error creating the message status: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    debug!(
        "Successfully logged message status for SmsSid: {:?}",
        form.sms_sid
    );

    Ok((StatusCode::OK, "Message status logged"))
}
