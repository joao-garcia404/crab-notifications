use std::sync::Arc;

use crate::infra::amqp::AmqpConsumer;
use crate::tracing::{error, info};
use crate::workers::email::EmailWorker;

use amqprs::BasicProperties;
use amqprs::Deliver;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsumerError {
    #[error("Failed to decode notification")]
    DecodeError,

    #[error("Failed to parse notification")]
    ParseError,
}

pub async fn handle_sms_notification(
    _deliver: Deliver,
    _properties: BasicProperties,
    content: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error + Send>> {
    info!(
        "Handling SMS notification: {:?}",
        String::from_utf8(content)
    );
    // Process SMS notification
    Ok(())
}

pub async fn handle_push_notification(
    _deliver: Deliver,
    _properties: BasicProperties,
    content: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error + Send>> {
    info!(
        "Handling push notification: {:?}",
        String::from_utf8(content)
    );
    // Process push notification
    Ok(())
}

pub async fn start_consumers(
    config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let email_consumer = AmqpConsumer::new(
        &config.rabbitmq_host,
        config.rabbitmq_port,
        &config.rabbitmq_user,
        &config.rabbitmq_password,
        "organization-1.email",
    )
    .await?;

    tokio::spawn(async move {
        let worker = Arc::new(EmailWorker::new());

        let consumer = email_consumer
            .consume("email_consumer", move |d, p, c| {
                let worker = Arc::clone(&worker);
                async move { worker.handle(d, p, c).await }
            })
            .await;

        if let Err(err) = consumer {
            error!("Failed to start email consumer: {:?}", err);
        }
    });

    Ok(())
}
