pub enum NotificationType {
    Email,
    SMS,
    Push,
}

pub struct EmailNotification {
    pub email: String,
    pub subject: String,
    pub body: String,
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
