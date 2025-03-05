use thiserror::Error;

#[derive(Debug, Error)]
pub enum S3Error {
    Generic,
}
