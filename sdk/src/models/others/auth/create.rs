use chrono::Utc;
use sea_orm::prelude::DateTime;

#[derive(Debug)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub date_of_birth: Option<chrono::DateTime<Utc>>,
}
