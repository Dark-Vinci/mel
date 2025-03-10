use {
    crate::errors::s3::S3Error,
    async_trait::async_trait,
    aws_sdk_s3::operation::put_object::PutObjectOutput,
    std::{fmt::Display, str::FromStr},
    uuid::Uuid,
};

#[async_trait]
pub trait ObjectStore {
    async fn upload(&self, obj: Object) -> Result<PutObjectOutput, S3Error>;
    async fn get(&self, key: Uuid, bucket: String) -> Result<(), S3Error>;
}

pub enum ContentType {
    PDF,
    MP4,
    JSON,
}

impl FromStr for ContentType {
    type Err = String;

    //  todo: update accordingly
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application/pdf" => Ok(ContentType::PDF),
            _ => Ok(ContentType::JSON),
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::JSON => "application/json".to_string(),
            _ => "".to_string(),
        };

        write!(f, "{}", str)
    }
}

pub struct Object {
    pub bucket: String,
    pub key: String,
    pub name: String,
    pub content_type: ContentType,
    pub content: Vec<u8>,
}
