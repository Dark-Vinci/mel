mod connection;
mod grpc;
mod repo;
pub mod s3;

pub use {connection::*, grpc::*, repo::*};

pub type AppError = Box<dyn std::error::Error>;
pub type AppResult<T> = Result<T, AppError>;
pub type RepoResult<T> = Result<T, RepoError>;
