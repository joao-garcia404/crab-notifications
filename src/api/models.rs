use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmailNotificationRequest {
    #[validate(email)]
    pub recipient: String,
    pub notification_type: String,
    pub product_id: String,
    pub template_id: Option<String>,
    pub subject: Option<String>,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreateNotificationResponse {
    pub id: String,
}
