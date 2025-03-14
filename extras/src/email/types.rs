use crate::email::email::{
    Email, EmailContents, EmailData, EmailError, IntermediateString,
};

pub enum EmailBody {
    Verify { link: String },

    Reset { link: String, user_name: String },

    Welcome,
}

pub mod html {
    use crate::email::types::EmailBody;

    pub fn get_html_body(body: EmailBody) -> String {
        match body {
            EmailBody::Verify { link } => {
                format!(include_str!("templates/verify.html"), link = link)
            },

            EmailBody::Reset { link, user_name } => {
                format!(
                    include_str!("templates/reset.html"),
                    link = link,
                    username = user_name
                )
            },

            EmailBody::Welcome => {
                include_str!("templates/welcome_to_community.html").to_string()
            },
        }
    }
}

pub struct VerifyEmail {
    pub recipient_email: String,
    pub subject: &'static str,
    pub auth_id: Option<String>,
    pub theme_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmailToken {
    email: String,
    exp: u64,
    entity: Option<String>,
}

impl EmailToken {
    pub async fn new_token(_email: String) -> Result<String, ()> {
        todo!()
    }

    pub fn get_email(&self) -> Result<String, ()> {
        todo!()
    }

    pub fn get_entity(&self) -> Option<&String> {
        todo!()
    }
}

pub fn get_link_with_token(
    base_url: String,
    token: String,
    action: String,
    auth_id: &Option<String>,
    theme_id: &Option<String>,
) -> String {
    let mut email_url = format!("{base_url}/user/{action}?token={token}");
    if let Some(auth_id) = auth_id {
        email_url = format!("{email_url}&auth_id={auth_id}");
    }
    if let Some(theme_id) = theme_id {
        email_url = format!("{email_url}&theme_id={theme_id}");
    }

    email_url
}

#[async_trait::async_trait]
impl EmailData for VerifyEmail {
    async fn get_email_data(
        &self,
        base_url: String,
    ) -> Result<EmailContents, EmailError> {
        let token = EmailToken::new_token(self.recipient_email.clone())
            .await
            .map_err(|_| EmailError::TokenGenerationFailure)?;

        let verify_email_link = get_link_with_token(
            base_url,
            token,
            "verify_email".into(),
            &self.auth_id,
            &self.theme_id,
        );

        let body = html::get_html_body(EmailBody::Verify {
            link: verify_email_link,
        });

        Ok(EmailContents {
            subject: self.subject.to_string(),
            body: IntermediateString::new(body),
            recipient: Email(self.recipient_email.clone()),
        })
    }
}
