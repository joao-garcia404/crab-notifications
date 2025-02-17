use api::routes::create_router;
use config::get_config;
use infra::amqp::AmqpPublisher;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{error, info, Tracing};

pub mod api;
pub mod config;
pub mod domain;
pub mod infra;
pub mod templates;
pub mod tracing;
pub mod workers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Tracing::init();

    let config = get_config();

    info!("Initing RabbitMQ publisher");

    let publisher = AmqpPublisher::new(
        &config.rabbitmq_host,
        config.rabbitmq_port,
        &config.rabbitmq_user,
        &config.rabbitmq_password,
        "notifications",
    )
    .await
    .map_err(|err| {
        error!("Failed to init RabbitMQ publisher: {}", err);
        err
    })?;

    publisher
        .setup_queues("organization-1", &["email", "sms", "push"])
        .await
        .map_err(|err| {
            error!("Failed to setup queues: {}", err);
            err
        })?;

    info!("RabbitMQ publisher inited");

    tokio::spawn(async move {
        info!("Starting consumers");

        let _ = infra::consumer::start_consumers(config)
            .await
            .map_err(|err| {
                error!("Failed to start consumers: {}", err);
                std::process::exit(1);
            });

        info!("Consumers started");
    });

    let app = create_router(publisher).layer(TraceLayer::new_for_http());

    let listener_address = format!("0.0.0.0:{}", config.port);

    let listener = tokio::net::TcpListener::bind(&listener_address)
        .await
        .expect("Failed to bind listener");

    info!("Server started on port {}", config.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
