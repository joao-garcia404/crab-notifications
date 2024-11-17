use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

pub enum NotificationType {
    Email,
    SMS,
    Push,
}

#[derive(Debug, Serialize)]
pub struct EmailNotification {
    pub id: String,
    pub template_id: String,
    pub recipient: String,
    pub subject: String,
    pub body: String,
    pub created_at: String,
}

impl EmailNotification {
    pub fn new(template_id: String, recipient: String, subject: String, body: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            template_id,
            recipient,
            subject,
            body,
            created_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn into_json_string(self) -> Result<String, Box<dyn std::error::Error>> {
        let json_content = serde_json::to_string(&self);

        json_content.map_err(|err| err.into())
    }
}

pub struct SMSNotification {
    pub phone_number: String,
    pub message: String,
}

pub struct PushNotification {
    pub device_token: String,
    pub module: Option<String>,
    pub title: String,
    pub description: Option<String>,
}
