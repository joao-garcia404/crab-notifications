use axum::{
    routing::{get, post},
    Router,
};

use crate::infra::amqp::AmqpPublisher;

use super::handlers;

pub fn create_router(publisher: AmqpPublisher) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .route(
            "/organizations/:org_id/email-notification",
            post(handlers::create_email_notification),
        )
        .with_state(publisher)
}

async fn healthcheck() -> &'static str {
    "OK"
}
