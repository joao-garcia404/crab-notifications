use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, Channel, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};

use serde::Serialize;
use std::sync::Arc;
use tokio::sync::OnceCell;

static CONNECTION: OnceCell<Arc<Connection>> = OnceCell::const_new();
static CHANNEL: OnceCell<Arc<Channel>> = OnceCell::const_new();

#[derive(Clone)]
pub struct AmqpPublisher {
    exchange: String,
    channel: Arc<Channel>,
}

impl AmqpPublisher {
    pub async fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        exchange: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let connection = match CONNECTION.get() {
            Some(conn) => conn.clone(),
            None => {
                let connection = Arc::new(
                    Connection::open(&OpenConnectionArguments::new(
                        host, port, username, password,
                    ))
                    .await?,
                );

                connection
                    .register_callback(DefaultConnectionCallback)
                    .await?;
                CONNECTION
                    .set(connection.clone())
                    .map_err(|_err| "Failed to set connection in the OnceCell".to_string())?;
                connection
            }
        };

        let channel = match CHANNEL.get() {
            Some(ch) => ch.clone(),
            None => {
                let channel = Arc::new(connection.open_channel(None).await?);
                channel.register_callback(DefaultChannelCallback).await?;
                CHANNEL
                    .set(channel.clone())
                    .map_err(|_err| "Failed to set channel in the OnceCell".to_string())?;

                channel
            }
        };

        channel
            .exchange_declare(amqprs::channel::ExchangeDeclareArguments::new(
                exchange, "topic",
            ))
            .await?;

        Ok(Self {
            exchange: exchange.to_string(),
            channel,
        })
    }

    pub async fn setup_queues(
        &self,
        org_id: &str,
        notification_types: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for notification_type in notification_types {
            let queue_name = format!("{}.{}", org_id, notification_type);
            let routing_key = format!("{}.{}#", org_id, notification_type);

            self.channel
                .queue_declare(QueueDeclareArguments::new(&queue_name))
                .await?;

            self.channel
                .queue_bind(QueueBindArguments::new(
                    &queue_name,
                    &self.exchange,
                    &routing_key,
                ))
                .await?;
        }

        Ok(())
    }

    pub async fn publish<T: Serialize>(
        &self,
        routing_key: &str,
        payload: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::to_vec(payload)?;

        self.channel
            .basic_publish(
                BasicProperties::default(),
                payload,
                BasicPublishArguments::new(&self.exchange, routing_key),
            )
            .await?;

        Ok(())
    }
}
