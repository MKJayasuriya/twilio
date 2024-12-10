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

pub async fn handle_twilio_webhook_status(
    Extension(app_state): ExtState,
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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "PascalCase")]
pub struct WebhookPayload {
    pub sms_message_sid: String,
    pub num_media: i32,
    pub profile_name: String,
    pub message_type: String,
    pub sms_sid: String,
    pub wa_id: String,
    pub sms_status: String,
    pub body: String,
    pub button_text: Option<String>,
    pub to: String,
    pub button_payload: String,
    pub num_segments: i32,
    pub referral_num_media: i32,
    pub message_sid: String,
    pub account_sid: String,
    pub from: String,
    pub api_version: String,
}

pub async fn handle_twilio_webhook_payload(
    Extension(app_state): ExtState,
    Form(form): Form<WebhookPayload>,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    let db: &PgPool = &app_state.db;

    tracing::info!("resp {:?}", form);

    let query = r#"
        INSERT INTO twilio_incoming_messages (
            sms_message_sid,
            num_media,
            profile_name,
            message_type,
            sms_sid,
            wa_id,
            sms_status,
            body,
            button_text,
            to_phone,
            button_payload,
            num_segments,
            referral_num_media,
            message_sid,
            account_sid,
            from_phone,
            api_version
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
        )
    "#;

    match sqlx::query(query)
        .bind(form.sms_message_sid)
        .bind(form.num_media)
        .bind(form.profile_name)
        .bind(form.message_type)
        .bind(form.sms_sid)
        .bind(form.wa_id)
        .bind(form.sms_status)
        .bind(form.body)
        .bind(form.button_text)
        .bind(form.to)
        .bind(form.button_payload)
        .bind(form.num_segments)
        .bind(form.referral_num_media)
        .bind(form.message_sid)
        .bind(form.account_sid)
        .bind(form.from)
        .bind(form.api_version)
        .execute(db)
        .await
    {
        Ok(_) => {
            println!("Incoming message logged successfully");
            Ok((axum::http::StatusCode::OK, "Message logged"))
        }
        Err(e) => {
            eprintln!("Error logging incoming message: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
