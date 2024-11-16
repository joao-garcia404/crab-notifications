use axum::{extract::Path, http::StatusCode, Json};
use tracing::{info, warn};
use validator::Validate;

use crate::domain::notification::EmailNotification;

use super::models::{CreateEmailNotificationRequest, CreateNotificationResponse};

pub async fn create_email_notification(
    Path(org_id): Path<String>,
    Json(payload): Json<CreateEmailNotificationRequest>,
) -> Result<Json<CreateNotificationResponse>, StatusCode> {
    let _validation_result = payload.validate().map_err(|err| {
        warn!(
            "Invalid email notification request payload: {:?}, error: {:?}",
            payload, err
        );
        StatusCode::BAD_REQUEST
    })?;

    info!("Received email notification request for org_id: {}", org_id);
    info!("Email notification payload: {:?}", payload);

    // Find the email template
    // let html_template =

    // let notification = EmailNotification {
    //     email: payload.recipient,
    //     body: payload.content,
    //     subject: payload
    //         .subject
    //         .unwrap_or_else(|| "Notification".to_string()),
    // };

    Ok(Json(CreateNotificationResponse {
        id: "123".to_string(),
    }))
}
