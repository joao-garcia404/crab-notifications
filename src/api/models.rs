use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmailNotificationRequest {
    #[validate(length(min = 1, message = "Organization ID is required"))]
    pub organization_id: String,
    #[validate(email(message = "Invalid e-mail"))]
    pub recipient: String,
    #[validate(length(min = 1, message = "Template ID is required"))]
    pub template_id: String,
    pub subject: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct CreateNotificationResponse {
    pub id: String,
}
