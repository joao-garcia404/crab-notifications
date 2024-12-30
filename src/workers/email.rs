use std::sync::Arc;

use amqprs::{BasicProperties, Deliver};
use resend_rs::{types::CreateEmailBaseOptions, Resend};

use crate::{
    domain::notification::{EmailNotification, Notification},
    infra::consumer::ConsumerError,
    templates::email::{
        engine::EmailTemplateEngine,
        repository::{EmailTemplateRepository, FileEmailTemplateRepository},
    },
    tracing::{error, info},
};

pub struct EmailWorker {
    repository: Arc<dyn EmailTemplateRepository>,
    engine: Arc<EmailTemplateEngine>,
    resend: Arc<Resend>,
}

impl EmailWorker {
    pub fn new() -> Self {
        let repository = Arc::new(FileEmailTemplateRepository::new("templates".to_string()));
        let engine = Arc::new(EmailTemplateEngine::new());
        let resend = Arc::new(Resend::default());

        Self {
            repository,
            engine,
            resend,
        }
    }

    pub async fn handle(
        &self,
        _deliver: Deliver,
        _properties: BasicProperties,
        content: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error + Send>> {
        info!("Consuming email notification");

        let json_content = String::from_utf8(content).map_err(|err| {
            error!("Failed to decode email notification: {:?}", err);

            Box::new(ConsumerError::DecodeError) as Box<dyn std::error::Error + Send>
        })?;

        info!("Json email notification: {:?}", &json_content);

        let notification = EmailNotification::from_json_string(&json_content).map_err(|err| {
            error!("Failed to parse email notification: {:?}", err);
            Box::new(ConsumerError::ParseError) as Box<dyn std::error::Error + Send>
        })?;

        info!("Parsed email notification: {:?}", notification);

        let template = self
            .repository
            .find_by_id(&notification.template_id)
            .await
            .map_err(|err| {
                error!("Failed to find email template: {:?}", err);
                Box::new(ConsumerError::ParseError) as Box<dyn std::error::Error + Send>
            })?;

        let rendered = self
            .engine
            .render(&template, &notification.metadata)
            .map_err(|err| {
                error!("Failed to render email notification: {:?}", err);
                Box::new(ConsumerError::ParseError) as Box<dyn std::error::Error + Send>
            })?;

        info!("Rendered email notification: {:?}", rendered);

        let from = "Crab <onboarding@resend.dev>";
        let to = [notification.recipient];
        let subject = template.subject;
        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&rendered);

        let _email = self.resend.emails.send(email).await.map_err(|err| {
            error!("Failed to send email notification: {:?}", err);
            Box::new(ConsumerError::ParseError) as Box<dyn std::error::Error + Send>
        })?;

        info!("Email for notification {} sent", notification.id);

        Ok(())
    }
}
