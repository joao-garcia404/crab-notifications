use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    NotFound(String),

    #[error("Failed to render template: {0}")]
    RenderError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: String,
    pub subject: String,
    pub body: String,
}
