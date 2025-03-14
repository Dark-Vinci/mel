use {
    crate::email::email::{Email, EmailClient, EmailError, IntermediateString},
    async_trait::async_trait,
};

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct NoEmailClient {}

impl NoEmailClient {
    pub async fn create() -> Self {
        Self {}
    }
}

#[async_trait]
impl EmailClient for NoEmailClient {
    type RichText = String;

    async fn send(
        &self,
        _recipient: Email,
        _subject: String,
        _body: Self::RichText,
    ) -> Result<(), EmailError> {
        Ok(())
    }

    async fn to_text(
        &self,
        intermediate_string: IntermediateString,
    ) -> Result<String, EmailError>
    where
        Self::RichText: Send,
    {
        Ok(intermediate_string.into_inner())
    }
}
