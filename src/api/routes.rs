use axum::{
    extract::FromRef,
    routing::{get, post},
    Json, Router,
};

use crate::infra::amqp::AmqpPublisher;

use super::handlers;

#[derive(Clone)]
pub struct AppState {
    pub publisher: AmqpPublisher,
}

impl FromRef<AppState> for AmqpPublisher {
    fn from_ref(state: &AppState) -> AmqpPublisher {
        state.publisher.clone()
    }
}

pub type HttpResponse<T> = Json<T>;

pub fn create_router(publisher: AmqpPublisher) -> Router {
    let app_state = AppState { publisher };

    Router::new()
        .route("/healthcheck", get(healthcheck))
        .route(
            "/email-notification",
            post(handlers::create_email_notification),
        )
        .with_state(app_state)
}

async fn healthcheck() -> &'static str {
    "OK"
}
