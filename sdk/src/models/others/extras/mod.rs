use uuid::Uuid;

#[derive(Debug)]
pub struct CreateShortUrl {

}

#[derive(Debug)]
pub struct CreateShortUrlTrack {

}

#[derive(Debug)]
pub struct CreateProfileMedia{
    pub url: String,
    pub workspace_user_id: Uuid,
}