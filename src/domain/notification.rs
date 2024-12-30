use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Failed to parse notification")]
    ParseError(#[from] serde_json::Error),
}

pub trait Notification: Serialize + for<'de> Deserialize<'de> {
    fn into_json_string(&self) -> Result<String, NotificationError> {
        let json_content = serde_json::to_string(&self);

        json_content.map_err(|err| err.into())
    }

    fn from_json_string(json: &str) -> Result<Self, NotificationError>
    where
        Self: Sized,
    {
        let notification: Self = serde_json::from_str(json)?;

        Ok(notification)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailNotification {
    pub id: String,
    pub template_id: String,
    pub recipient: String,
    pub created_at: String,
    pub metadata: serde_json::Value,
}

impl Notification for EmailNotification {}

impl EmailNotification {
    pub fn new(template_id: String, recipient: String, metadata: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            template_id,
            recipient,
            created_at: Utc::now().to_rfc3339(),
            metadata,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SMSNotification {
    pub phone_number: String,
    pub message: String,
}

impl Notification for SMSNotification {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushNotification {
    pub device_token: String,
    pub module: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

impl Notification for PushNotification {}
