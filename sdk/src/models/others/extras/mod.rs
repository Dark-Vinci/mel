use uuid::Uuid;

#[derive(Debug)]
pub struct CreateShortUrl {}

#[derive(Debug)]
pub struct CreateShortUrlTrack {}

#[derive(Debug)]
pub struct CreateProfileMedia {
    pub url: String,
    pub workspace_user_id: Uuid,
}

#[derive(Debug)]
pub struct CreateChatMedia {
    pub url: String,
    pub channel_id: Option<Uuid>,
    pub message_id: Uuid,
}

#[derive(Debug)]
pub struct CreateSearch {}

#[derive(Debug)]
pub struct CreateAuditLogs {}

#[derive(Debug)]
pub struct CreateHistory {}

pub struct CreateEmail{}