use handlebars::Handlebars;
use serde_json::Value;

use super::template::{EmailTemplate, TemplateError};

pub struct EmailTemplateEngine {
    handlebars: Handlebars<'static>,
}

impl EmailTemplateEngine {
    pub fn new() -> Self {
        Self {
            handlebars: Handlebars::new(),
        }
    }

    pub fn render(
        &self,
        template: &EmailTemplate,
        metadata: &Value,
    ) -> Result<String, TemplateError> {
        let email_content = self
            .handlebars
            .render_template(&template.body, metadata)
            .map_err(|err| TemplateError::RenderError(err.to_string()))?;

        Ok(email_content)
    }
}
