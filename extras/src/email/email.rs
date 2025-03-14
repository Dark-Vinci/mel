use {
    crate::email::providers::smtp::SmtpServerConfig, async_trait::async_trait,
};

#[derive(Debug)]
pub struct IntermediateString(String);

impl IntermediateString {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct Email(String);

#[derive(Debug)]
pub struct EmailContents {
    pub subject: String,

    pub body: IntermediateString,

    pub recipient: Email,
}

pub struct EmailSettings {
    pub sender_email: String,
    pub allowed_unverified_days: i64,
    pub client_config: EmailClientConfigs,
}

impl EmailSettings {
    pub(crate) fn new(
        sender_email: String,
        allowed_unverified_days: i64,
        client_config: EmailClientConfigs,
    ) -> Self {
        Self {
            sender_email,
            allowed_unverified_days,
            client_config,
        }
    }
}

pub enum EmailClientConfigs {
    #[default]
    None,

    Smtp {
        smtp: SmtpServerConfig,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Error building email client")] // can be smtp| ses
    BuildFailure,

    #[error("Error sending email")]
    SendingFailure,

    #[error("Not implemented")]
    NotImplemented,

    #[error("Unable to generate verification token")]
    TokenGenerationFailure,
}

#[async_trait]
pub trait EmailClient {
    type RichText;

    async fn send(
        &self,
        recipient: Email,
        subject: String,
        body: Self::RichText,
    ) -> Result<(), EmailError>;

    async fn to_text(
        &self,
        intermediate_string: IntermediateString,
    ) -> Result<String, EmailError>
    where
        Self::RichText: Send;
}

#[async_trait]
pub trait EmailService: Sync + Send {
    async fn compose_and_send_email(
        &self,
        base_url: &str,
        email_data: Box<dyn EmailData + Send>,
        proxy_url: Option<&String>,
    ) -> Result<(), EmailError>;
}

#[async_trait]
pub trait EmailData {
    async fn get_email_data(
        &self,
        base_url: &str,
    ) -> Result<EmailContents, EmailError>;
}
