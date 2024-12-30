use async_trait::async_trait;
use tokio::fs;

use super::template::{EmailTemplate, TemplateError};

use crate::tracing::error;

#[async_trait]
pub trait EmailTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<EmailTemplate, TemplateError>;
}

pub struct FileEmailTemplateRepository {
    templates_path: String,
}

impl FileEmailTemplateRepository {
    pub fn new(templates_path: String) -> Self {
        Self { templates_path }
    }
}

#[async_trait]
impl EmailTemplateRepository for FileEmailTemplateRepository {
    async fn find_by_id(&self, id: &str) -> Result<EmailTemplate, TemplateError> {
        let path = format!("{}/{}.json", self.templates_path, id);

        let content = fs::read_to_string(&path).await.map_err(|err| {
            error!("Failed to read email template file: {:?}", err);

            TemplateError::NotFound(id.to_string())
        })?;

        let template: EmailTemplate = serde_json::from_str(&content)
            .map_err(|e| TemplateError::RenderError(e.to_string()))?;

        Ok(template)
    }
}
