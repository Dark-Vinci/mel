use std::time::Duration;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use crate::email::email::{Email, EmailClient, EmailError, EmailSettings, IntermediateString};

#[derive(Default, Clone)]
pub enum SmtpConnection {
    #[default]
    StartTls,
    Plaintext
}

#[derive(Clone)]
pub struct SmtpServerConfig {
    pub host: String,
    pub port: u16,
    pub time_out: u64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub connection: SmtpConnection,
}

impl SmtpServerConfig {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.host.is_empty() {
            return Err("email.smtp.host must not be empty");
        }

        if self.username.is_none()  {
            return Err("email.smtp.username must not be empty");
        }

        if self.password.is_none()  {
            return Err("email.smtp.password must not be empty");
        }

        Ok(())
    }
}


pub struct SmtpServer {
    pub sender: String,
    pub smtp_config: SmtpServerConfig,
}

impl EmailClient for SmtpServer {
    type RichText = String;

    async fn send(
        &self,
        recipient: Email,
        subject: String,
        body: Self::RichText
    ) -> Result<(), EmailError> {
        // Create a client every time when the email is being sent
        let email_client =
            Self::create_client(self).map_err(|_| EmailError::SendingFailure)?;

        let email = Message::builder()
            .to(Self::parse_mail(recipient.to_string())?)
            .from(Self::parse_mail(self.sender.clone())?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)
            .map_err(SmtpError::EmailParsingFailed)
            .map_err(EmailError::SendingFailure)?;

        email_client
            .send(&email)
            .map_err(SmtpError::SendingFailure)
            .map_err(EmailError::SendingFailure)?;
        Ok(())
    }

    async fn to_text(&self, intermediate_string: IntermediateString) -> Result<String, EmailError>
    where
        Self::RichText: Send
    {
        Ok(intermediate_string.into_inner())
    }
}

impl SmtpServer {
    pub fn create(conf: &EmailSettings, smtp_config: SmtpServerConfig) -> Self {
        Self {
            sender: conf.sender_email.clone(),
            smtp_config: smtp_config.clone(),
        }
    }

    fn parse_mail(email: String) -> Result<Mailbox, EmailError> {
        Ok(Mailbox::new(
            None,
            email
                .parse()
                .map_err(|_| EmailError::SendingFailure)?
        ))
    }

    fn create_client(&self) -> Result<SmtpTransport, SmtpError> {
        let host = self.smtp_config.host.clone();
        let port = self.smtp_config.port;
        let timeout = Some(Duration::from_secs(self.smtp_config.time_out));

        let credentials = self
            .smtp_config
            .username
            .clone()
            .zip(self.smtp_config.password.clone())
            .map(|(username, password)| {
                Credentials::new(username, password)
            });

        match &self.smtp_config.connection {
            SmtpConnection::StartTls => {
                match credentials {
                    Some(credentials) => {
                        Ok(
                            SmtpTransport::starttls_relay(&host)
                                .map_err(|_| SmtpError::ConnectionFailure)?
                                .port(port)
                                .timeout(timeout)
                                .credentials(credentials)
                                .build(),
                        )
                    }

                    None => {
                        Ok(SmtpTransport::starttls_relay(&host)
                            .map_err(|_| SmtpError::ConnectionFailure)?
                            .port(port)
                            .timeout(timeout)
                            .build()
                        )
                    }
                }
            }

            SmtpConnection::Plaintext => match credentials {
                Some(credentials) => Ok(SmtpTransport::builder_dangerous(&host)
                    .port(port)
                    .timeout(timeout)
                    .credentials(credentials)
                    .build()),

                None => Ok(SmtpTransport::builder_dangerous(&host)
                    .port(port)
                    .timeout(timeout)
                    .build()),
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SmtpError {
    SendingFailure,
    ConnectionFailure,
    EmailParsingFailed,
}