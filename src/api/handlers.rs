use amqprs::{channel::BasicPublishArguments, BasicProperties};
use axum::{extract::State, http::StatusCode, Json};
use tracing::{info, warn};
use validator::Validate;

use crate::{
    api::errors::HttpError,
    domain::notification::{EmailNotification, Notification},
    infra::amqp::AmqpPublisher,
};

use super::{
    models::{CreateEmailNotificationRequest, CreateNotificationResponse},
    routes::HttpResponse,
};

pub async fn create_email_notification(
    State(publisher): State<AmqpPublisher>,
    Json(payload): Json<CreateEmailNotificationRequest>,
) -> Result<HttpResponse<CreateNotificationResponse>, HttpError> {
    let _validation_result = payload.validate().map_err(|err| {
        warn!(
            "Invalid email notification request payload: {:?}, error: {:?}",
            payload, err
        );

        return HttpError {
            status_code: StatusCode::BAD_REQUEST,
            message: format!("Invalid payload: {}", err),
        };
    })?;

    info!(
        "Received email notification request for organization: {}",
        payload.organization_id
    );

    info!("Email notification payload: {:?}", payload);

    let notification =
        EmailNotification::new(payload.template_id, payload.recipient, payload.metadata);

    let json_content = notification.into_json_string().map_err(|err| {
        warn!("Failed to serialize email notification: {:?}", err);

        HttpError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
        }
    })?;

    let routing_key = format!("{}.email", payload.organization_id);

    let publish_args = BasicPublishArguments::new(&publisher.exchange, &routing_key);

    publisher
        .channel
        .basic_publish(
            BasicProperties::default(),
            json_content.into_bytes(),
            publish_args,
        )
        .await
        .map_err(|err| {
            warn!("Failed to publish email notification: {:?}", err);

            HttpError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error".to_string(),
            }
        })?;

    info!("Email notification published successfully");

    Ok(Json(CreateNotificationResponse {
        id: notification.id,
    }))
}
