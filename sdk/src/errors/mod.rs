mod repo;
mod connection;
mod grpc;

pub use repo::*;
pub use connection::*;
pub use grpc::*;

pub type AppError = Box<dyn std::error::Error>;
pub type AppResult<T> = Result<T, AppError>;
pub type RepoResult<T> = Result<T, RepoError>;
