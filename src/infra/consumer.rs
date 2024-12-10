use crate::infra::amqp::AmqpConsumer;
use amqprs::BasicProperties;
use amqprs::Deliver;
use tracing::info;

pub async fn handle_email_notification(
    _deliver: Deliver,
    _properties: BasicProperties,
    content: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error + Send>> {
    info!(
        "Handling email notification: {:?}",
        String::from_utf8(content)
    );

    // Process email notification
    Ok(())
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

    // Will be added soon

    // let sms_consumer = AmqpConsumer::new(
    //     &config.rabbitmq_host,
    //     config.rabbitmq_port,
    //     &config.rabbitmq_user,
    //     &config.rabbitmq_password,
    //     "organization-1.sms",
    // )
    // .await?;

    // let push_consumer = AmqpConsumer::new(
    //     &config.rabbitmq_host,
    //     config.rabbitmq_port,
    //     &config.rabbitmq_user,
    //     &config.rabbitmq_password,
    //     "organization-1.push",
    // )
    // .await?;

    tokio::spawn(async move {
        email_consumer
            .consume("email_consumer", handle_email_notification)
            .await
            .unwrap();
    });

    // Will be added soon

    // tokio::spawn(async move {
    //     sms_consumer
    //         .consume("sms_consumer", handle_sms_notification)
    //         .await
    //         .unwrap();
    // });

    // tokio::spawn(async move {
    //     push_consumer
    //         .consume("push_consumer", handle_push_notification)
    //         .await
    //         .unwrap();
    // });

    Ok(())
}
