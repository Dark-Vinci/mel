use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum S3Error {
    Generic,
}

impl Display for S3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
