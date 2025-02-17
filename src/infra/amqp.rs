use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicNackArguments, BasicPublishArguments, Channel,
        QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    BasicProperties, Deliver,
};

use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::error;

static CONNECTION: OnceCell<Arc<Connection>> = OnceCell::const_new();
static CHANNEL: OnceCell<Arc<Channel>> = OnceCell::const_new();

#[derive(Clone)]
pub struct AmqpPublisher {
    pub exchange: String,
    pub channel: Arc<Channel>,
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
            let routing_key = format!("{}.{}", org_id, notification_type);

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

#[derive(Clone)]
pub struct AmqpConsumer {
    pub channel: Arc<Channel>,
    pub queue: String,
}

impl AmqpConsumer {
    pub async fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        queue: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
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

        Ok(Self {
            channel,
            queue: queue.to_string(),
        })
    }

    pub async fn consume<F, Fut>(
        &self,
        consumer_tag: &str,
        callback: F,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(Deliver, BasicProperties, Vec<u8>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send>>>
            + Send
            + 'static,
    {
        let args = BasicConsumeArguments::new(&self.queue, consumer_tag)
            .manual_ack(true)
            .finish();

        struct AsyncConsumer<F, Fut> {
            callback: F,
            _fut: std::marker::PhantomData<Fut>,
        }

        impl<F, Fut> AsyncConsumer<F, Fut>
        where
            F: Fn(Deliver, BasicProperties, Vec<u8>) -> Fut + Send + Sync + 'static,
            Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send>>>
                + Send
                + 'static,
        {
            fn new(callback: F) -> Self {
                Self {
                    callback,
                    _fut: std::marker::PhantomData,
                }
            }
        }

        #[async_trait]
        impl<F, Fut> amqprs::consumer::AsyncConsumer for AsyncConsumer<F, Fut>
        where
            F: Fn(Deliver, BasicProperties, Vec<u8>) -> Fut + Send + Sync + 'static,
            Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send>>>
                + Send
                + 'static,
        {
            async fn consume(
                &mut self,
                channel: &Channel,
                deliver: Deliver,
                basic_properties: BasicProperties,
                content: Vec<u8>,
            ) {
                let delivery_tag = deliver.delivery_tag();

                if let Err(err) = (self.callback)(deliver, basic_properties, content).await {
                    error!("Failed to consume message: {:?}", err);

                    let nack_args = BasicNackArguments::new(delivery_tag, false, true);

                    if let Err(err) = channel.basic_nack(nack_args).await {
                        error!("Failed to nack message: {:?}", err);
                    }
                } else {
                    let ack_args = amqprs::channel::BasicAckArguments::new(delivery_tag, false);

                    if let Err(err) = channel.basic_ack(ack_args).await {
                        error!("Failed to ack message: {:?}", err);
                    }
                }
            }
        }

        let consumer = AsyncConsumer::new(callback);

        self.channel.basic_consume(consumer, args).await?;

        Ok(())
    }
}
